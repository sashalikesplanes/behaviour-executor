#![no_std]

use core::convert::Infallible;

use itsybitsy_m4::hal::rtc;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds_trait::RGB8;

pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

pub fn main_loop(
    mut debug_led: impl embedded_hal::digital::v2::ToggleableOutputPin<Error = Infallible>,
    count_timer: rtc::Rtc<rtc::Count32Mode>,
    mut neopixels: [impl smart_leds_trait::SmartLedsWrite<Error = (), Color = RGB8>; 1],
) -> ! {
    let mut loop_counter: u32 = 0;
    loop {
        loop_counter += 1;
        if loop_counter % 10 == 0 {
            debug_led.toggle().unwrap();
        }

        let timer_count: u32 = count_timer.count32();

        let colors = [hsv2rgb(Hsv {
            hue: 69,
            sat: 255,
            val: 100,
        }); 400];
        neopixels[0].write(colors.iter().cloned()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
