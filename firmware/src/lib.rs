#![no_std]

use micromath::F32Ext;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds_trait::RGB8;

pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Strips {
    // Two strips of 100 LEDs each
    // TODO dyncamically derive from config
    pub strips: ([RGB8; 100], [RGB8; 100]),
}

struct Event {}
struct Events {}

pub fn calculate_new_strips(timer_counter: u32) -> Strips {
    return Strips {
        strips: (
            constant_color_strip_100(
                hsv2rgb(Hsv {
                    hue: (timer_counter as f32 / 100.0).rem_euclid(255.0) as u8,
                    sat: 255,
                    val: 50,
                }),
                10,
                40,
            ),
            constant_color_strip_100(
                hsv2rgb(Hsv {
                    hue: ((timer_counter as f32 * 0.001).sin() * 255.0) as u8,
                    sat: 255,
                    val: 50,
                }),
                20,
                60,
            ),
        ),
    };
}

fn constant_color_strip_100(color: RGB8, start_index: usize, end_index: usize) -> [RGB8; 100] {
    let mut colors = [RGB8 { r: 0, g: 0, b: 0 }; 100];

    for (i, current_color) in colors.iter_mut().enumerate() {
        if i >= start_index && i <= end_index {
            *current_color = color;
        }
    }

    return colors;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_color_strip() {
        let result = constant_color_strip_100(RGB8 { r: 100, g: 0, b: 0 }, 0, 1);

        assert_eq!(result[0], RGB8 { r: 100, g: 0, b: 0 });
        assert_eq!(result[1], RGB8 { r: 100, g: 0, b: 0 });
        assert_eq!(result[2], RGB8 { r: 0, g: 0, b: 0 });
        assert_eq!(result[98], RGB8 { r: 0, g: 0, b: 0 });
        assert_eq!(result[99], RGB8 { r: 0, g: 0, b: 0 });

        assert_eq!(result.len(), 100);
    }
}
