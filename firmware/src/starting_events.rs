use heapless::Vec;
use crate::structs::{EventWrapper, Event, MessageEvent};

pub fn add_starting_events(events: &mut Vec<EventWrapper, 2048>, timer_count: u32) -> () {
    events.push(EventWrapper {
        event: Event::Message(MessageEvent {
            color: [100, 100, 100],
            pace: 0.01,
            message_width: 5,
            strip_idx: 0,
            start_idx: 0,
            end_idx: 100,
            start_node: 0,
            end_node: 0,
        }),
        finished: false,
        start_time: timer_count,
        active: true,
    });
    events.push(EventWrapper {
        event: Event::Message(MessageEvent {
            color: [100, 100, 100],
            pace: 0.001,
            message_width: 5,
            strip_idx: 0,
            start_idx: 0,
            end_idx: 100,
            start_node: 0,
            end_node: 0,
        }),
        finished: false,
        start_time: timer_count,
        active: false,
    });
}
