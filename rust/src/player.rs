use godot::classes::{
    AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, InputEvent, InputEventKey,
};
use godot::global::Key;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    speed: f32,
    direction: Vector2,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 320.0,
            direction: Vector2::ZERO,
            base,
        }
    }

    fn ready(&mut self) {
        let mut animation_node = self
            .base_mut()
            .get_node_as::<AnimatedSprite2D>("CharacterBody2D/AnimatedSprite2D");

        animation_node.set_animation("front_idle".into());
    }

    fn physics_process(&mut self, delta: f64) {
        let position = self.base().get_position();

        let direction = self.direction;
        let speed = self.speed;

        self.base_mut()
            .set_position(position + (direction * speed * delta as f32));

        let mut animation_node = self
            .base_mut()
            .get_node_as::<AnimatedSprite2D>("CharacterBody2D/AnimatedSprite2D");

        match self.direction {
            Vector2::UP => animation_node.set_animation("back_idle".into()),
            Vector2::RIGHT => animation_node.set_animation("right_idle".into()),
            Vector2::DOWN => animation_node.set_animation("front_idle".into()),
            Vector2::LEFT => animation_node.set_animation("left_idle".into()),
            Vector2::ZERO => animation_node.set_animation("front_idle".into()),
            _ => {}
        };

        animation_node.play();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        match event.try_cast::<InputEventKey>() {
            Ok(e) => {
                if e.is_pressed() {
                    match e.get_keycode() {
                        Key::W => self.direction = Vector2::UP,
                        Key::A => self.direction = Vector2::LEFT,
                        Key::S => self.direction = Vector2::DOWN,
                        Key::D => self.direction = Vector2::RIGHT,
                        _ => {}
                    };
                } else {
                    self.direction = Vector2::ZERO;
                }
            }
            Err(_) => {}
        }
    }
}
