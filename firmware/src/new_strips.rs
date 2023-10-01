use crate::{
    behaviours::{constant_color_strip_200, paint_message_event},
    structs::{Duration, EventWrapper},
};
use heapless::Vec;
use smart_leds_trait::RGB8;

const STRIP_INDICES: (usize, usize) = (0, 1);
pub const STRIP_LENGTH: usize = 200;

#[derive(Copy, Clone)]
pub struct Strips {
    // Two strips of 100 LEDs each
    // TODO dyncamically derive from config
    pub strips: ([RGB8; STRIP_LENGTH], [RGB8; STRIP_LENGTH]),
}

pub fn calculate_new_strips(
    timer_counter: u32,
    active_events: &mut Vec<EventWrapper, 2048>,
) -> Strips {
    // TODO if we have a clear event, we can skip and delete all other events
    update_events(timer_counter, active_events);

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
            crate::structs::Event::Message(e) => paint_message_event(
                &mut strips.strips.0,
                e,
                event.start_time.unwrap(),
                timer_counter,
            ),
            // TODO deal with constant event
            _ => {}
        }
    }

    strips
}

fn update_events(timer_counter: u32, active_events: &mut Vec<EventWrapper, 2048>) -> () {
    // activate next events
    let mut mut_events_iter = active_events.iter_mut().peekable();
    while let Some(event) = mut_events_iter.next() {
        if event.finished(timer_counter) {
            if let Some(next_event) = mut_events_iter.peek_mut() {
                if !next_event.active() {
                    next_event.activate(timer_counter);
                }
            }
        }
    }

    active_events.retain(|event| !event.finished(timer_counter));
}
