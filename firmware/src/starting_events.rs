use crate::{
    new_strips::{MAX_EVENTS, STRIP_INDICES},
    structs::{ConstantEvent, Event, EventWrapper, MessageEvent},
};
use heapless::Vec;
use micromath::F32Ext;
use smart_leds_trait::RGB8;

pub fn add_starting_events(events: &mut Vec<EventWrapper, MAX_EVENTS>, timer_count: f32) -> () {
    events.push(EventWrapper {
        event: Event::Message(MessageEvent {
            color: RGB8 { r: 100, g: 0, b: 0 },
            pace: 20.0,
            message_width: 7,
            strip_idx: STRIP_INDICES.0,
            start_idx: 0,
            end_idx: 99,
        }),
        start_time: Some(timer_count),
    });
    events.push(EventWrapper {
        event: Event::Message(MessageEvent {
            color: RGB8 { r: 100, g: 0, b: 0 },
            pace: 20.0,
            message_width: 7,
            strip_idx: STRIP_INDICES.0,
            start_idx: 99,
            end_idx: 0,
        }),
        start_time: None,
    });
    for i in 0..2 {
        events.push(EventWrapper {
            event: Event::Message(MessageEvent {
                color: RGB8 { r: 100, g: 0, b: 0 },
                pace: 20.0,
                message_width: 7,
                strip_idx: STRIP_INDICES.0,
                start_idx: 0,
                end_idx: 99,
            }),
            start_time: None,
        });
        events.push(EventWrapper {
            event: Event::Message(MessageEvent {
                color: RGB8 { r: 100, g: 0, b: 0 },
                pace: 20.0,
                message_width: 7,
                strip_idx: STRIP_INDICES.0,
                start_idx: 99,
                end_idx: 0,
            }),
            start_time: None,
        });
    }
    events.push(EventWrapper {
        event: Event::Message(MessageEvent {
            color: RGB8 { r: 100, g: 0, b: 0 },
            pace: 20.0,
            message_width: 7,
            strip_idx: STRIP_INDICES.1,
            start_idx: 0,
            end_idx: 99,
        }),
        start_time: Some(timer_count),
    });
    events.push(EventWrapper {
        event: Event::Message(MessageEvent {
            color: RGB8 { r: 100, g: 0, b: 0 },
            pace: 20.0,
            message_width: 7,
            strip_idx: STRIP_INDICES.1,
            start_idx: 99,
            end_idx: 0,
        }),
        start_time: None,
    });
    for i in 0..2 {
        events.push(EventWrapper {
            event: Event::Message(MessageEvent {
                color: RGB8 { r: 100, g: 0, b: 0 },
                pace: 20.0,
                message_width: 7,
                strip_idx: STRIP_INDICES.1,
                start_idx: 0,
                end_idx: 99,
            }),
            start_time: None,
        });
        events.push(EventWrapper {
            event: Event::Message(MessageEvent {
                color: RGB8 { r: 100, g: 0, b: 0 },
                pace: 20.0,
                message_width: 7,
                strip_idx: STRIP_INDICES.1,
                start_idx: 99,
                end_idx: 0,
            }),
            start_time: None,
        });
    }
    //     events.push(EventWrapper {
    //         event: Event::Message(MessageEvent {
    //             color: RGB8 { r: 0, g: 0, b: 100 },
    //             pace: 0.01,
    //             message_width: 7,
    //             strip_idx: 0,
    //             start_idx: 49,
    //             end_idx: 0,
    //         }),
    //         start_time: None,
    //     });
}
