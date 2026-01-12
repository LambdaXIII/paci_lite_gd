use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=Node)]
pub struct PaciCore {
    base: Base<Node>,
}

#[godot_api]
impl PaciCore {
    #[func]
    pub fn test(&self) {
        godot_print!("PaciCore test");
    }
}
