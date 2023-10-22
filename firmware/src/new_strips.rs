use crate::{
    behaviours::{paint_message_event, paint_solid_pixel},
    structs::{Duration, EventWrapper},
};
use heapless::Vec;
use micromath::F32Ext;
use smart_leds_trait::RGB8;

// pub const STRIP_INDICES: (usize, usize) = (2, 5);
// pub const SERIAL_NUM: &str = "IB_0_";
// pub const STRIP_INDICES: (usize, usize) = (7, 0);
// pub const SERIAL_NUM: &str = "IB_1_";
// pub const STRIP_INDICES: (usize, usize) = (4, 6);
// pub const SERIAL_NUM: &str = "IB_2_";
pub const STRIP_INDICES: (usize, usize) = (3, 1);
pub const SERIAL_NUM: &str = "IB_3_";
pub const CLOCK_MULTIPLIER: f32 = 1.0 / 1024.0;
pub const STRIP_LENGTH: usize = 200;
pub const MAX_EVENTS: usize = 2047;

#[derive(Copy, Clone)]
pub struct Strips {
    pub strips: ([RGB8; STRIP_LENGTH], [RGB8; STRIP_LENGTH]),
}

pub fn calculate_new_strips(
    timer_seconds: f32,
    active_events: &mut Vec<EventWrapper, MAX_EVENTS>,
) -> Strips {
    // TODO if we have a clear event, we can skip and delete all other events
    update_events(timer_seconds, active_events);

    let mut strips = Strips {
        strips: (
            [RGB8 { r: 0, g: 0, b: 0 }; STRIP_LENGTH],
            [RGB8 { r: 0, g: 0, b: 0 }; STRIP_LENGTH],
        ),
    };

    let mut events_iter = active_events.iter();
    while let Some(event) = events_iter.next() {
        if !event.active() {
            continue;
        }

        match &event.event {
            crate::structs::Event::Message(e) => {
                if e.strip_idx == STRIP_INDICES.0 {
                    paint_message_event(
                        &mut strips.strips.0,
                        e,
                        event.start_time.unwrap(),
                        timer_seconds,
                    )
                } else if e.strip_idx == STRIP_INDICES.1 {
                    paint_message_event(
                        &mut strips.strips.1,
                        e,
                        event.start_time.unwrap(),
                        timer_seconds,
                    )
                }
            },
            crate::structs::Event::Constant(e) => {
                if e.strip_idx == STRIP_INDICES.0 {
                    paint_solid_pixel(&mut strips.strips.0, e, event.start_time.unwrap(), timer_seconds);
                } else if e.strip_idx == STRIP_INDICES.1 {
                    paint_solid_pixel(&mut strips.strips.1, e, event.start_time.unwrap(), timer_seconds);
                }
            }
            // TODO deal with constant event
            _ => {}
        }
    }

    strips
}

fn update_events(timer_seconds: f32, active_events: &mut Vec<EventWrapper, MAX_EVENTS>) -> () {
    // activate next events
    let mut mut_events_iter = active_events.iter_mut().peekable();
    while let Some(event) = mut_events_iter.next() {
        if event.finished(timer_seconds) {
            if let Some(next_event) = mut_events_iter.peek_mut() {
                if !next_event.active() {
                    next_event.activate(timer_seconds);
                }
            }
        }
    }

    active_events.retain(|event| !event.finished(timer_seconds));
}
