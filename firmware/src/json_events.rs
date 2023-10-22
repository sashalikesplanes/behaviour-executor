use crate::{
    new_strips::{MAX_EVENTS, STRIP_INDICES},
    structs::{ConstantEvent, Event, EventWrapper, MessageEvent},
};
use heapless::Vec;
use micromath::F32Ext;
use microjson::{JSONValue, JSONValueType};
use smart_leds_trait::RGB8;

pub fn add_events_from_json(
    events: &mut Vec<EventWrapper, MAX_EVENTS>,
    json_str: &str,
    timer_seconds: f32,
) -> () {
    let json = JSONValue::parse(json_str).unwrap();
    match json.get_key_value("type").unwrap().read_string().unwrap() {
        "message" => {
            process_message_node(&json, timer_seconds, events, true);
        }
        "clear" => {
            events.clear();
        }
        "constant" => {
            process_constant_node(&json, timer_seconds, events, true);
        }
        _ => panic!("Unknown event type"),
    };
}

fn process_message_node(
    node: &JSONValue,
    timer_seconds: f32,
    events: &mut Vec<EventWrapper, MAX_EVENTS>,
    first_node: bool,
) {
    if node.value_type == JSONValueType::Null {
        return;
    }

    let strip_idx: usize = node
        .get_key_value("strip_idx")
        .unwrap()
        .read_integer()
        .unwrap() as usize;

    if strip_idx == STRIP_INDICES.0 || strip_idx == STRIP_INDICES.1 {
        events.push(EventWrapper {
            start_time: if first_node { Some(timer_seconds) } else { None },
            event: Event::Message(parse_message_event(&node)),
        });
    }

    let next = node.get_key_value("next").unwrap();
    process_message_node(&next, timer_seconds, events, false);
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

fn process_constant_node(
    node: &JSONValue,
    timer_seconds: f32,
    events: &mut Vec<EventWrapper, MAX_EVENTS>,
    first_node: bool,
) {
    // constant events never have a next
    if node.value_type == JSONValueType::Null {
        return;
    }

    let color: Vec<u8, 3> = node
        .get_key_value("color")
        .unwrap()
        .iter_array()
        .unwrap()
        .map(|x| x.read_integer().unwrap() as u8)
        .collect();

    let duration: f32 = node
        .get_key_value("duration")
        .unwrap()
        .read_integer()
        .unwrap() as f32;
    let fadein_duration: u32 = node
        .get_key_value("fadein_duration")
        .unwrap()
        .read_integer()
        .unwrap() as u32;
    let fadeout_duration: u32 = node
        .get_key_value("fadeout_duration")
        .unwrap()
        .read_integer()
        .unwrap() as u32;

    // loop over the pixels array of the json
    node.get_key_value("pixels")
        .unwrap()
        .iter_array()
        .unwrap()
        .for_each(|pixel| {
            let pixel_idx: usize = pixel
                .get_key_value("pixel_idx")
                .unwrap()
                .read_integer()
                .unwrap() as usize;
            let strip_idx: usize = pixel
                .get_key_value("strip_idx")
                .unwrap()
                .read_integer()
                .unwrap() as usize;

            if strip_idx == STRIP_INDICES.0 || strip_idx == STRIP_INDICES.1 {
                events.push(EventWrapper {
                    start_time: Some(timer_seconds),
                    event: Event::Constant(ConstantEvent {
                        color: RGB8 {
                            r: color[0],
                            g: color[1],
                            b: color[2],
                        },
                        duration,
                        fadein_duration,
                        fadeout_duration,
                        fade_power: 0,
                        pixel_idx,
                        strip_idx,
                    }),
                });
            }
        })
}
