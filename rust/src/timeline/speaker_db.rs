use super::speaker::Speaker;
use godot::prelude::*;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(init,base=Resource)]
pub struct SpeakerDB {
    speakers: HashMap<StringName, Gd<Speaker>>,
    shortcuts: HashMap<StringName, String>,

    base: Base<Resource>,
}

#[godot_api]
impl SpeakerDB {
    #[func]
    fn get_speaker(&self, speaker_id: StringName) -> Option<Gd<Speaker>> {
        self.speakers.get(&speaker_id).cloned()
    }

    #[func]
    fn get_id_by_shortcut(&self, shortcut: StringName) -> Option<GString> {
        self.shortcuts
            .get(&shortcut)
            .map(|s| GString::from(s))
            .into()
    }

    #[func]
    fn new_speaker_by_id(&self, speaker_id: StringName) -> Gd<Speaker> {
        let mut speaker = Speaker::new_gd();
        speaker.bind_mut().speaker_id = speaker_id;
        speaker
    }

    // #[func]
    // fn set_shortcut(&mut self, shortcut: StringName, speaker_id: String) {
    //     self.shortcuts.insert(shortcut, speaker_id);
    // }
}
