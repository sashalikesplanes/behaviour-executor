#[derive(Copy, Clone)]
pub struct Pixel {
    pub strip_idx: u8,
    pub pixel_idx: u16,
}

pub struct ConstantEvent {
    pub color: [u8; 3],
    pub duration: u32,
    pub fadein_duration: u32,
    pub fadeout_duration: u32,
    pub fade_power: u8,
    pub pixels: [Pixel; 10], // Fixed size array due to stack allocation
}

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

pub struct ClearEvent;

pub enum Event {
    Message(MessageEvent),
    Clear(ClearEvent),
    Constant(ConstantEvent),
}

pub struct EventWrapper {
    pub event: Event,
    pub active: bool,
    pub finished: bool,
    pub start_time: u32,
}
