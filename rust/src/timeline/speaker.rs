use godot::classes::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=Resource)]
pub struct Speaker {
    #[export]
    pub name: GString,

    #[export]
    pub description: GString,

    #[export]
    pub color: Color,

    #[var]
    pub speaker_id: StringName,

    base: Base<Resource>,
}

// #[godot_api]
// impl Speaker {
//     #[signal]
//     fn info_changed();
// }
