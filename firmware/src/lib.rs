#![no_std]

use heapless::Vec;
use micromath::F32Ext;
use smart_leds_trait::RGB8;

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub strip_idx: u8,
    pub pixel_idx: u16,
}

#[derive(Debug)]
pub struct ConstantEvent {
    pub color: [u8; 3],
    pub duration: u32,
    pub fadein_duration: u32,
    pub fadeout_duration: u32,
    pub fade_power: u8,
    pub pixels: [Pixel; 10], // Fixed size array due to stack allocation
}

#[derive(Debug)]
pub struct MessageEvent {
    pub color: [u8; 3],
    pub message_width: u16,
    pub pace: f32,
    pub strip_idx: u8,
    pub start_idx: usize,
    pub end_idx: usize,
    pub start_node: u8,
    pub end_node: u8,
}

#[derive(Debug)]
pub struct ClearEvent;

#[derive(Debug)]
pub enum Event {
    Message(MessageEvent),
    Clear(ClearEvent),
    Constant(ConstantEvent),
}

#[derive(Debug)]
pub struct EventWrapper {
    pub event: Event,
    pub active: bool,
    pub finished: bool,
    pub start_time: u32,
}

pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

const INTENSITY_THESHOLD: f32 = 0.05;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Strips {
    // Two strips of 100 LEDs each
    // TODO dyncamically derive from config
    pub strips: ([RGB8; 200], [RGB8; 200]),
}

pub fn calculate_new_strips(
    timer_counter: u32,
    color: RGB8,
    active_events: &mut Vec<EventWrapper, 2048>,
) -> Strips {
    return Strips {
        strips: (
            message(0, timer_counter, active_events),
            constant_color_strip_200(RGB8 { r: 0, g: 0, b: 0 }, 0, 200),
        ),
    };
}

fn message(
    strip_idx: u8,
    timer_counter: u32,
    active_events: &mut Vec<EventWrapper, 2048>,
) -> [RGB8; 200] {
    let mut colors = [RGB8 { r: 0, g: 0, b: 0 }; 200];
    let mut mut_events_iter = active_events.iter_mut().peekable();
    while let Some(event) = mut_events_iter.next() {
        if !event.active {
            continue;
        }

        match &mut event.event {
            Event::Message(e) => {
                let duration = (((e.end_idx - e.start_idx) as f32).abs() + 1.0 + e.message_width as f32 / 2.0) / e.pace;
                if event.active && timer_counter > event.start_time + duration as u32 {
                    // TODO, we can remove this event from the active events
                    event.active = false;
                    event.finished = true;
                    mut_events_iter.peek_mut().map(|next_event| {
                        next_event.start_time = timer_counter;
                        next_event.active = true;
                    });
                    continue;
                }
                let position = (timer_counter - event.start_time) as f32 * e.pace;

                let (start_idx, end_idx) = if e.start_idx < e.end_idx {
                    (e.start_idx, e.end_idx)
                } else {
                    (e.end_idx, e.start_idx)
                };

                for idx in start_idx..end_idx {
                    if (idx as f32 - position).abs() > e.message_width as f32 / 2.0 {
                        continue;
                    }

                    let intensity = get_message_pixel_intensity(
                        (timer_counter - event.start_time) as u32,
                        idx as f32,
                        e,
                    );

                    colors[idx as usize] = RGB8 {
                        r: (e.color[0] as f32 * intensity).round() as u8,
                        g: (e.color[1] as f32 * intensity).round() as u8,
                        b: (e.color[2] as f32 * intensity).round() as u8,
                    };
                }
            }
            _ => {}
        }
    }
    colors
}

//def get_intensity(elapsed_time, pixel_offset):
// if pixel_offset > elapsed_time * message_config["pace"]:
// return 0
// if pixel_offset < elapsed_time * message_config["pace"] - message_config["message_width"] / 2:
// return 0
// return -sin((pixel_offset - elapsed_time * message_config["pace"]) / message_config["message_width"] * pi * 2)

fn get_message_pixel_intensity(elapsed_time: u32, pixel_offset: f32, event: &MessageEvent) -> f32 {
    if pixel_offset > elapsed_time as f32 * event.pace as f32 + event.message_width as f32 / 2.0 {
        return 0.0;
    }
    if pixel_offset < elapsed_time as f32 * event.pace as f32 - event.message_width as f32 / 2.0 {
        return 0.0;
    }
    return -(pixel_offset - elapsed_time as f32 * event.pace as f32).sin()
        / event.message_width as f32
        * 2.0
        * 3.141592;
}

fn constant_color_strip_200(color: RGB8, start_index: usize, end_index: usize) -> [RGB8; 200] {
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
