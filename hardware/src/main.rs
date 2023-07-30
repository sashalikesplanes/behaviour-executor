#![no_std]
#![no_main]

use bsp::hal::timer::TimerCounter;
use bsp::hal::{self, rtc};
use itsybitsy_m4 as bsp;
use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use smart_leds_trait::SmartLedsWrite;
use ws2812_timer_delay::Ws2812;
use panic_halt as _;
use firmware::calculate_new_strips;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let _ = CorePeripherals::take().unwrap();
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

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timers = (TimerCounter::tc2_(&timer_clock, peripherals.TC2, &mut peripherals.MCLK), TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK));
    // DOCS say that this should be 3MHz, but it seems to work also with 6MHz, maybe better
    timers.0.start(6.mhz());
    timers.1.start(6.mhz());

    let mut neopixels = (Ws2812::new(timers.0, pins.d2.into_push_pull_output()), Ws2812::new(timers.1, pins.d3.into_push_pull_output()));

    let mut loop_counter: u32 = 0;
    loop {
        loop_counter += 1;
        if loop_counter % 10 == 0 {
            debug_led.toggle().unwrap();
        }

        let timer_count: u32 = count_timer.count32();
        let strips = calculate_new_strips(timer_count);
        neopixels.0.write(strips.strips.0.iter().cloned()).unwrap();
        neopixels.1.write(strips.strips.1.iter().cloned()).unwrap();
    }
}
