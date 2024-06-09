use godot::prelude::*;
use godot::engine::{ISprite2D, Sprite2D};
use godot::obj::Base;

#[derive(GodotClass)]
#[class(base=ISprite2D)]
pub struct Player {
  base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player {

  fn init(base: Base<Sprite2D>) -> Self {
    Self { base }
  }

  fn physics_process(&mut self, delta: f32) {
    let radians: f32 = (3.14 * delta) as f32;
    self.base_mut().rotate(radians);
  }
}