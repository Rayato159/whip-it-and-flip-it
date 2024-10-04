use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

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
    key_state: RefCell<HashMap<Key, bool>>,
    keys: Rc<Vec<Key>>,
    base: Base<CharacterBody2D>,
    animation_node: Option<RefCell<Gd<AnimatedSprite2D>>>,
}

#[godot_api]
impl Player {
    #[func]
    fn walk(&mut self, velocity: Vector2) {
        let position = self.base_mut().get_position();
        let transition = position + velocity;

        self.base_mut().set_position(transition);
        self.base_mut().move_and_slide();
    }

    #[func]
    fn init_animation_node(&mut self) {
        let animation_node = self
            .base_mut()
            .get_node_as::<AnimatedSprite2D>("CharacterBody2D/AnimatedSprite2D");

        self.animation_node = Some(RefCell::new(animation_node));
    }

    #[func]
    fn set_animation(&mut self, animation: StringName) {
        if let Some(animation_node) = &self.animation_node {
            animation_node.borrow_mut().set_animation(animation);
        }
    }

    #[func]
    fn play_animation(&mut self) {
        if let Some(animation_node) = &self.animation_node {
            animation_node.borrow_mut().play();
        }
    }

    #[func]
    fn change_direction_animation(&mut self) {
        if let Some(animation_node) = &self.animation_node {
            match self.direction {
                Vector2::UP => {
                    animation_node
                        .borrow_mut()
                        .set_animation("back_idle".into());
                }
                Vector2::RIGHT => {
                    animation_node
                        .borrow_mut()
                        .set_animation("right_idle".into());
                }
                Vector2::DOWN => {
                    animation_node
                        .borrow_mut()
                        .set_animation("front_idle".into());
                }
                Vector2::LEFT => {
                    animation_node
                        .borrow_mut()
                        .set_animation("left_idle".into());
                }
                _ => {}
            }

            animation_node.borrow_mut().play();
        }
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 300.0,
            direction: Vector2::ZERO,
            key_state: RefCell::new(HashMap::from([
                (Key::W, false),
                (Key::D, false),
                (Key::S, false),
                (Key::A, false),
            ])),
            keys: Rc::new(vec![Key::W, Key::D, Key::S, Key::A]),
            base,
            animation_node: None,
        }
    }

    fn ready(&mut self) {
        self.init_animation_node();
        self.set_animation("front_idle".into());
    }

    fn physics_process(&mut self, delta: f64) {
        godot_print!("{:?}", self.key_state.borrow());

        let keys = Rc::clone(&self.keys);
        let key_state = &self.key_state;

        if keys
            .iter()
            .all(|k| !key_state.borrow().get(k).unwrap_or(&false))
        {
            self.direction = Vector2::ZERO;
            return;
        }

        keys.iter().for_each(|k| {
            let state = match key_state.borrow_mut().get(k) {
                Some(state) => *state,
                None => return,
            };

            if state {
                match *k {
                    Key::W => self.direction = Vector2::UP,
                    Key::D => self.direction = Vector2::RIGHT,
                    Key::S => self.direction = Vector2::DOWN,
                    Key::A => self.direction = Vector2::LEFT,
                    _ => {}
                };
            }
        });

        self.walk(self.direction * self.speed * delta as f32);
        self.change_direction_animation();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let keys = Rc::clone(&self.keys);
        let key_state = &self.key_state;

        match event.try_cast::<InputEventKey>() {
            Ok(e) => {
                if e.is_pressed() {
                    keys.iter().for_each(|k| {
                        if e.get_keycode() == *k {
                            key_state.borrow_mut().insert(*k, true);
                        }
                    });
                }

                if e.is_released() {
                    keys.iter().for_each(|k| {
                        if e.get_keycode() == *k {
                            key_state.borrow_mut().insert(*k, false);
                        }
                    });
                }
            }
            Err(_) => {}
        }
    }
}
