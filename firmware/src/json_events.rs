use heapless::Vec;
use microjson::JSONValue;
use crate::structs::{EventWrapper, Event, ClearEvent, ConstantEvent, MessageEvent, Pixel};

pub fn add_events_from_json(
    events: &mut Vec<EventWrapper, 2048>,
    json_str: &str,
    timer_count: u32,
) -> () {
    let json = JSONValue::parse(json_str).unwrap();
    events.push(
        match json.get_key_value("type").unwrap().read_string().unwrap() {
            "clear" => EventWrapper {
                event: Event::Clear(ClearEvent),
                start_time: Some(timer_count),
            },
            "constant" => {
                // let color: Vec<u8, 3> = json.get_key_value("color").unwrap().iter_array().unwrap().map(|x| x.read_integer().unwrap() as u8).collect();
                // let duration = json.get_key_value("duration").unwrap().read_integer().unwrap() as u32;
                // let fadein_duration = json.get_key_value("fadein_duration").unwrap().read_integer().unwrap() as u32;
                // let fadeout_duration = json.get_key_value("fadeout_duration").unwrap().read_integer().unwrap() as u32;
                // let fade_power = json.get_key_value("fade_power").unwrap().read_integer().unwrap() as u32;
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
                }
            }
            "message" => {
                let color: Vec<u8, 3> = json
                    .get_key_value("color")
                    .unwrap()
                    .iter_array()
                    .unwrap()
                    .map(|x| x.read_integer().unwrap() as u8)
                    .collect();

                EventWrapper {
                    start_time: Some(timer_count),
                    event: Event::Message(MessageEvent {
                        color: smart_leds_trait::RGB { r: color[0], g: color[1], b: color[2] },
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
                        start_node: json
                            .get_key_value("start_node")
                            .unwrap()
                            .read_integer()
                            .unwrap() as u8,
                        end_node: json
                            .get_key_value("end_node")
                            .unwrap()
                            .read_integer()
                            .unwrap() as u8,
                    }),
                }
            }
            _ => panic!("Unknown event type"),
        },
    );
}
