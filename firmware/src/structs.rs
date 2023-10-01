use micromath::F32Ext;
use smart_leds_trait::RGB8;

#[derive(Copy, Clone)]
pub struct Pixel {
    pub strip_idx: u8,
    pub pixel_idx: u16,
}

pub struct ConstantEvent {
    pub color: RGB8,
    pub duration: u32,
    pub fadein_duration: u32,
    pub fadeout_duration: u32,
    pub fade_power: u8,
    pub pixels: [Pixel; 10], // Fixed size array due to stack allocation
}

pub struct MessageEvent {
    pub color: RGB8,
    pub message_width: u16,
    pub pace: f32,
    pub strip_idx: usize,
    pub start_idx: usize,
    pub end_idx: usize,
}

pub enum Event {
    Message(MessageEvent),
    Constant(ConstantEvent),
}

pub struct EventWrapper {
    pub event: Event,
    pub start_time: Option<u32>,
}

pub trait Duration {
    fn duration(&self) -> f32;
    fn active(&self) -> bool;
    fn finished(&self, timer_count: u32) -> bool;
    fn activate(&mut self, timer_count: u32);
}

impl Duration for EventWrapper {
    fn duration(&self) -> f32 {
        match &self.event {
            Event::Message(e) => {
                ((e.end_idx as f32 - e.start_idx as f32).abs() + 1.0 + e.message_width as f32 / 2.0)
                    / e.pace
            },
            Event::Constant(e) => e.duration as f32,
        }
    }

    fn active(&self) -> bool {
        self.start_time.is_some()
    }

    fn finished(&self, timer_count: u32) -> bool {
        if let Some(start_time) = self.start_time {
            timer_count as f32 > start_time as f32 + self.duration()
        } else {
            false
        }
    }

    fn activate(&mut self, timer_count: u32) {
        self.start_time = Some(timer_count);
    }
}
