use godot::classes::CharacterBody2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f64,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl CharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self { speed: 400.0, base }
    }
}
