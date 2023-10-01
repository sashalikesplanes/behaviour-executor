use heapless::Vec;
use smart_leds_trait::RGB8;
use crate::{structs::EventWrapper, behaviours::{message, constant_color_strip_200}};

#[derive(Copy, Clone)]
pub struct Strips {
    // Two strips of 100 LEDs each
    // TODO dyncamically derive from config
    pub strips: ([RGB8; 200], [RGB8; 200]),
}


pub fn calculate_new_strips(
    timer_counter: u32,
    active_events: &mut Vec<EventWrapper, 2048>,
) -> Strips {
    // Here iterate over the events,
    // create a color per strip and pass it to each event executor which will paint onto it
    // also here deal with next event logic
    return Strips {
        strips: (
            message(0, timer_counter, active_events),
            constant_color_strip_200(RGB8 { r: 0, g: 0, b: 0 }, 0, 200),
        ),
    };
}

