#![no_std]
#![no_main]

use arrform::{arrform, ArrForm};
use bsp::entry;
use bsp::hal::timer::TimerCounter;
use bsp::hal::{self, rtc, usb::UsbBus};
use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;
use firmware::json_events::add_events_from_json;
use firmware::new_strips::{calculate_new_strips, MAX_EVENTS, SERIAL_NUM};
use firmware::starting_events::add_starting_events;
use firmware::structs::EventWrapper;
use hal::clock::GenericClockController;
use hal::pac::interrupt;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use heapless::Vec;
use itsybitsy_m4 as bsp;
use panic_halt as _;
use smart_leds_trait::SmartLedsWrite;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use ws2812_timer_delay::Ws2812;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut debug_led = pins.d13.into_push_pull_output();

    // Need to make it first a clock, then into a count32 mode in order for the counter to start
    // WHY??? I don't know
    let count_timer = rtc::Rtc::clock_mode(peripherals.RTC, 1024.hz(), &mut peripherals.MCLK);
    let count_timer: rtc::Rtc<rtc::Count32Mode> = count_timer.into_count32_mode();

    // Neopixel setup
    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timers = (
        TimerCounter::tc2_(&timer_clock, peripherals.TC2, &mut peripherals.MCLK),
        TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK),
    );
    // DOCS say that this should be 3MHz, but it seems to work also with 6MHz, maybe better
    timers.0.start(6.mhz());
    timers.1.start(6.mhz());
    let mut neopixels = (
        Ws2812::new(timers.0, pins.d2.into_push_pull_output()),
        Ws2812::new(timers.1, pins.d3.into_push_pull_output()),
    );

    // USB setup
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.MCLK,
            pins.usb_dm,
            pins.usb_dp,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number(SERIAL_NUM)
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB_OTHER, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT0, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        NVIC::unmask(interrupt::USB_OTHER);
        NVIC::unmask(interrupt::USB_TRCPT0);
        NVIC::unmask(interrupt::USB_TRCPT1);
    }

    // Logging example
    cortex_m::interrupt::free(|_| unsafe {
        if USB_BUS.as_mut().is_some() {
            if let Some(serial) = USB_SERIAL.as_mut() {
                let _ = serial.write("Starting up\n".as_bytes());
            }
        }
    });

    // Flash the LED every 10 loops
    let mut loop_counter: u32 = 0;
    loop {
        loop_counter += 1;
        if loop_counter % 10 == 0 {
            debug_led.toggle().unwrap();
        }

        let timer_count: u32 = count_timer.count32();

        if loop_counter == 100 {
            unsafe {
                add_starting_events(&mut ACTIVE_EVENTS, timer_count);
            }
        }

        // This should be safe, as disable_interrupts stops USB interrupts (only place which uses JSON_BUF)
        // and only the main loop uses ACTIVE_EVENTS
        disable_interrupts(|_| unsafe {
            let mut pos = 0;
            while pos < JSON_BUF_LEN {
                if JSON_BUF[pos] == b'\n' {
                    let json_str = core::str::from_utf8(&JSON_BUF[0..=pos]).unwrap();

                    add_events_from_json(&mut ACTIVE_EVENTS, json_str, timer_count);

                    JSON_BUF.copy_within(pos + 1..JSON_BUF_LEN, 0);
                    JSON_BUF_LEN -= pos + 1;
                    pos = 0; // Reset the position
                } else {
                    pos += 1;
                }
            }
        });
        // This should be safe as only the main loop uses ACTIVE_EVENTS
        let strips = unsafe { calculate_new_strips(timer_count, &mut ACTIVE_EVENTS) };
        neopixels.0.write(strips.strips.0.iter().cloned()).unwrap();
        neopixels.1.write(strips.strips.1.iter().cloned()).unwrap();
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

// Only for main thread
static mut ACTIVE_EVENTS: Vec<EventWrapper, MAX_EVENTS> = Vec::new();

// Shared between main and USB interrupts
const MAX_JSON_LEN: usize = 4096;
static mut JSON_BUF: [u8; MAX_JSON_LEN] = [0; MAX_JSON_LEN];
static mut JSON_BUF_LEN: usize = 0;

fn poll_usb() {
    disable_interrupts(|_| unsafe {
        if let Some(usb_dev) = USB_BUS.as_mut() {
            if let Some(serial) = USB_SERIAL.as_mut() {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        JSON_BUF[JSON_BUF_LEN] = *c;
                        JSON_BUF_LEN += 1;
                    }
                };
            };
        };
    });
}

#[interrupt]
fn USB_OTHER() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT0() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT1() {
    poll_usb();
}
