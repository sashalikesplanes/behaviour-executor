use core::f32::consts::PI;

use crate::structs::{Event, EventWrapper, MessageEvent};
use crate::{new_strips::STRIP_LENGTH, structs::Duration};
use heapless::Vec;
use micromath::F32Ext;
use smart_leds_trait::RGB8;

const INTENSITY_THESHOLD: f32 = 0.05;

pub fn paint_message_event(
    strip: &mut [RGB8; STRIP_LENGTH],
    event: &MessageEvent,
    start_time: u32,
    timer_counter: u32,
) -> () {
    let event_position = (timer_counter - start_time) as f32 * event.pace;

    if event.start_idx < event.end_idx {
        for idx in event.start_idx..event.end_idx {
            let pixel_position = (idx - event.start_idx) as f32;
            let intensity = get_message_pixel_intensity(pixel_position, event_position, event);
            let current_color = strip[idx as usize];
            strip[idx as usize] = RGB8 {
                r: (event.color.r as f32 * intensity + current_color.r as f32).round().min(255.0) as u8,
                g: (event.color.g as f32 * intensity + current_color.g as f32).round().min(255.0) as u8,
                b: (event.color.b as f32 * intensity + current_color.b as f32).round().min(255.0) as u8,
            };
        }
    } else {
        for idx in event.end_idx..event.start_idx {
            let pixel_position = -(idx as f32 - event.start_idx as f32);
            let intensity = get_message_pixel_intensity(pixel_position, event_position, event);
            let current_color = strip[idx as usize];
            strip[idx as usize] = RGB8 {
                r: (event.color.r as f32 * intensity + current_color.r as f32).round().min(255.0) as u8,
                g: (event.color.g as f32 * intensity + current_color.g as f32).round().min(255.0) as u8,
                b: (event.color.b as f32 * intensity + current_color.b as f32).round().min(255.0) as u8,
            } + strip[idx as usize];
        }
    }
}

fn get_message_pixel_intensity(
    pixel_position: f32,
    event_position: f32,
    event: &MessageEvent,
) -> f32 {
    if pixel_position > event_position + event.message_width as f32 / 2.0 {
        return 0.0;
    }
    if pixel_position < event_position - event.message_width as f32 / 2.0 {
        return 0.0;
    }

    let intensity =
        (((pixel_position - event_position).abs() / event.message_width as f32 * 2.0) * PI / 2.0)
            .cos();
    return intensity.max(0.0).min(1.0);
}

pub fn constant_color_strip_200(color: RGB8, start_index: usize, end_index: usize) -> [RGB8; 200] {
    let mut colors = [RGB8 { r: 0, g: 0, b: 0 }; 200];

    for (i, current_color) in colors.iter_mut().enumerate() {
        if i >= start_index && i <= end_index && i % 5 == 0 {
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
        let result = constant_color_strip_200(RGB8 { r: 100, g: 0, b: 0 }, 0, 1);

        assert_eq!(result[0], RGB8 { r: 100, g: 0, b: 0 });
        assert_eq!(result[1], RGB8 { r: 100, g: 0, b: 0 });
        assert_eq!(result[2], RGB8 { r: 0, g: 0, b: 0 });
        assert_eq!(result[98], RGB8 { r: 0, g: 0, b: 0 });
        assert_eq!(result[99], RGB8 { r: 0, g: 0, b: 0 });

        assert_eq!(result.len(), 100);
    }
}
