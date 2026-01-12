use super::speaker::Speaker;
use cx_studio::core::{Time, TimeRangeMutableSupport, TimeRangeSupport, Timestamp};
use godot::classes::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=Resource)]
pub struct SubtitleClip {
    #[var]
    content: GString,

    #[var]
    speaker_id: StringName,

    start_time: Time,
    duration: Time,

    base: Base<Resource>,
}

impl TimeRangeSupport for SubtitleClip {
    fn start_time(&self) -> Time {
        self.start_time
    }

    fn duration_time(&self) -> Time {
        self.duration
    }
}

impl TimeRangeMutableSupport for SubtitleClip {
    fn set_start_time(&mut self, time: Time) {
        self.start_time = time;
    }

    fn set_duration_time(&mut self, time: Time) {
        self.duration = time;
    }
}

#[godot_api]
impl SubtitleClip {
    #[func]
    pub fn get_start_tc(&self) -> GString {
        let s = Timestamp::from_time(self.start_time).to_string();
        GString::from(&s)
    }

    #[func]
    pub fn get_duration_tc(&self) -> GString {
        let s = Timestamp::from_time(self.duration).to_string();
        GString::from(&s)
    }

    #[func]
    pub fn get_end_tc(&self) -> GString {
        let s = Timestamp::from_time(self.start_time + self.duration).to_string();
        GString::from(&s)
    }

    #[func]
    pub fn get_start_ms(&self) -> i64 {
        self.start_time.to_milliseconds() as i64
    }

    #[func]
    pub fn get_end_ms(&self) -> i64 {
        (self.start_time + self.duration).to_milliseconds() as i64
    }

    #[func]
    pub fn get_duration_ms(&self) -> i64 {
        self.duration.to_milliseconds() as i64
    }
}
