use godot::classes::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=Resource)]
pub struct Speaker {
    #[export]
    name: GString,

    #[export]
    color: Color,

    #[export]
    shortcut: GString,

    #[export]
    description: GString,

    base: Base<Resource>,
}
