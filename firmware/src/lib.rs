#![no_std]

use smart_leds::hsv::{Hsv, hsv2rgb};
use smart_leds_trait::RGB8;

pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

pub fn main_loop(
    neopixels: &mut [impl smart_leds_trait::SmartLedsWrite<Error = (), Color = RGB8>; 1],
    timer_count: u32,
) {
    let colors = [hsv2rgb(Hsv {
        hue: (timer_count % 255) as u8,
        sat: 255,
        val: 200,
    })];
    neopixels[0].write(colors.iter().cloned()).unwrap();
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
