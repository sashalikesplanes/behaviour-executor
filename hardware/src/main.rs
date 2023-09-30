#![no_std]
#![no_main]

use arrform::{arrform, ArrForm};
use bsp::entry;
use bsp::hal::timer::TimerCounter;
use bsp::hal::{self, rtc, usb::UsbBus};
use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;
use firmware::{
    calculate_new_strips, ClearEvent, ConstantEvent, Event, EventWrapper, MessageEvent, Pixel,
};
use hal::clock::GenericClockController;
use hal::pac::interrupt;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use heapless::Vec;
use itsybitsy_m4 as bsp;
use microjson::JSONValue;
use panic_halt as _;
use smart_leds_trait::{SmartLedsWrite, RGB8};
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
                .serial_number("TEST")
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

    let mut loop_counter: u32 = 0;
    let current_color = RGB8 {
        r: 100,
        g: 100,
        b: 100,
    };

    // Logging example
    cortex_m::interrupt::free(|_| unsafe {
        if USB_BUS.as_mut().is_some() {
            if let Some(serial) = USB_SERIAL.as_mut() {
                let _ = serial.write("Starting up\n".as_bytes());
            }
        }
    });

    let timer_count: u32 = count_timer.count32();
    unsafe {
        ACTIVE_EVENTS.push(EventWrapper {
            event: Event::Message(MessageEvent {
                color: [100, 100, 100],
                pace: 0.001,
                message_width: 5,
                strip_idx: 0,
                start_idx: 0,
                end_idx: 100,
                start_node: 0,
                end_node: 0,
            }),
            start_time: timer_count,
            active: true,
        });
    }

    loop {
        loop_counter += 1;
        if loop_counter % 10 == 0 {
            debug_led.toggle().unwrap();
        }
        let timer_count: u32 = count_timer.count32();

        disable_interrupts(|_| unsafe {
            let mut pos = 0;
            while pos < JSON_BUF_LEN {
                if JSON_BUF[pos] == b'\n' {
                    let json_str = core::str::from_utf8(&JSON_BUF[0..=pos]).unwrap();
                    let json = JSONValue::parse(json_str).unwrap();

                    ACTIVE_EVENTS.push(
                        match json.get_key_value("type").unwrap().read_string().unwrap() {
                            "clear" => EventWrapper {
                                event: Event::Clear(ClearEvent),
                                start_time: timer_count,
                                active: true,
                            },
                            "constant" => {
                                // let color: Vec<u8, 3> = json.get_key_value("color").unwrap().iter_array().unwrap().map(|x| x.read_integer().unwrap() as u8).collect();
                                // let duration = json.get_key_value("duration").unwrap().read_integer().unwrap() as u32;
                                // let fadein_duration = json.get_key_value("fadein_duration").unwrap().read_integer().unwrap() as u32;
                                // let fadeout_duration = json.get_key_value("fadeout_duration").unwrap().read_integer().unwrap() as u32;
                                // let fade_power = json.get_key_value("fade_power").unwrap().read_integer().unwrap() as u32;
                                EventWrapper {
                                    start_time: timer_count,
                                    event: Event::Constant(ConstantEvent {
                                        color: [0, 0, 0],
                                        duration: 0,
                                        fadein_duration: 0,
                                        fadeout_duration: 0,
                                        fade_power: 0,
                                        pixels: [Pixel {
                                            strip_idx: 0,
                                            pixel_idx: 0,
                                        }; 10],
                                    }),
                                    // construct
                                    active: true,
                                }
                            }
                            "message" => {
                                let color: Vec<u8, 3> = json
                                    .get_key_value("color")
                                    .unwrap()
                                    .iter_array()
                                    .unwrap()
                                    .map(|x| x.read_integer().unwrap() as u8)
                                    .collect();

                                EventWrapper {
                                    start_time: timer_count,
                                    event: Event::Message(MessageEvent {
                                        color: [color[0], color[1], color[2]],
                                        pace: json
                                            .get_key_value("pace")
                                            .unwrap()
                                            .read_float()
                                            .unwrap()
                                            as f32,
                                        message_width: json
                                            .get_key_value("message_width")
                                            .unwrap()
                                            .read_integer()
                                            .unwrap()
                                            as u16,
                                        strip_idx: json
                                            .get_key_value("strip_idx")
                                            .unwrap()
                                            .read_integer()
                                            .unwrap()
                                            as u8,
                                        start_idx: json
                                            .get_key_value("start_idx")
                                            .unwrap()
                                            .read_integer()
                                            .unwrap()
                                            as usize,
                                        end_idx: json
                                            .get_key_value("end_idx")
                                            .unwrap()
                                            .read_integer()
                                            .unwrap()
                                            as usize,
                                        start_node: json
                                            .get_key_value("start_node")
                                            .unwrap()
                                            .read_integer()
                                            .unwrap()
                                            as u8,
                                        end_node: json
                                            .get_key_value("end_node")
                                            .unwrap()
                                            .read_integer()
                                            .unwrap()
                                            as u8,
                                    }),
                                    active: true,
                                }
                            }
                            _ => panic!("Unknown event type"),
                        },
                    );

                    // now iterate over the json through the next keys adding the events to the
                    // event list

                    JSON_BUF.copy_within(pos + 1..JSON_BUF_LEN, 0);
                    JSON_BUF_LEN -= pos + 1;
                    pos = 0; // Reset the position
                } else {
                    pos += 1;
                }
            }
        });

        // This should be safe as only the main loop uses ACTIBE_EVENTS
        unsafe {
            let strips = calculate_new_strips(timer_count, current_color, &mut ACTIVE_EVENTS);

            neopixels.0.write(strips.strips.0.iter().cloned()).unwrap();
            neopixels.1.write(strips.strips.1.iter().cloned()).unwrap();
        }
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

// Only for main thread
static mut ACTIVE_EVENTS: Vec<EventWrapper, 2048> = Vec::new();

// Shared between main and USB interrupts
static mut JSON_BUF: [u8; 2048] = [0; 2048];
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
