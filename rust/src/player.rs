use godot::prelude::*;
use godot::engine::{Sprite2D, ISprite2D};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        Self { base }
    }

    fn process(&mut self, delta: f64) {
        godot_print!("hello from process {}", delta);
    }
}
