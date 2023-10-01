use crate::{
    new_strips::MAX_EVENTS,
    structs::{ConstantEvent, Event, EventWrapper, MessageEvent, Pixel},
};
use heapless::Vec;
use microjson::{JSONValue, JSONValueType};

pub fn add_events_from_json(
    events: &mut Vec<EventWrapper, MAX_EVENTS>,
    json_str: &str,
    timer_count: u32,
) -> () {
    let json = JSONValue::parse(json_str).unwrap();
    match json.get_key_value("type").unwrap().read_string().unwrap() {
        "message" => {
            process_message_node(&json, timer_count, events, true);
        }
        "clear" => {
            events.clear();
        }
        "constant" => {
            EventWrapper {
                start_time: Some(timer_count),
                event: Event::Constant(ConstantEvent {
                    color: smart_leds_trait::RGB { r: 0, g: 0, b: 0 },
                    duration: 0,
                    fadein_duration: 0,
                    fadeout_duration: 0,
                    fade_power: 0,
                    pixels: [Pixel {
                        strip_idx: 0,
                        pixel_idx: 0,
                    }; 10],
                }),
            };
        }
        _ => panic!("Unknown event type"),
    };
}

fn process_message_node(
    node: &JSONValue,
    timer_count: u32,
    events: &mut Vec<EventWrapper, MAX_EVENTS>,
    first_node: bool,
) {
    if node.value_type == JSONValueType::Null {
        return;
    }
    events.push(EventWrapper {
        start_time: if first_node { Some(timer_count) } else { None },
        event: Event::Message(parse_message_event(&node)),
    });

    let next = node.get_key_value("next").unwrap();
    process_message_node(&next, timer_count, events, false);
}

fn parse_message_event(json: &JSONValue) -> MessageEvent {
    let color: Vec<u8, 3> = json
        .get_key_value("color")
        .unwrap()
        .iter_array()
        .unwrap()
        .map(|x| x.read_integer().unwrap() as u8)
        .collect();

    MessageEvent {
        color: smart_leds_trait::RGB {
            r: color[0],
            g: color[1],
            b: color[2],
        },
        pace: json.get_key_value("pace").unwrap().read_float().unwrap() as f32,
        message_width: json
            .get_key_value("message_width")
            .unwrap()
            .read_integer()
            .unwrap() as u16,
        strip_idx: json
            .get_key_value("strip_idx")
            .unwrap()
            .read_integer()
            .unwrap() as usize,
        start_idx: json
            .get_key_value("start_idx")
            .unwrap()
            .read_integer()
            .unwrap() as usize,
        end_idx: json
            .get_key_value("end_idx")
            .unwrap()
            .read_integer()
            .unwrap() as usize,
    }
}
