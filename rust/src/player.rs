use std::cell::RefCell;
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
    idle_animation_map: Rc<HashMap<StringName, StringName>>,
    animation_state: Rc<RefCell<StringName>>,
}

#[godot_api]
impl Player {
    #[func]
    fn init_animation_node(&mut self) {
        let animation_node = self
            .base_mut()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

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
    fn play_walk_animation(&mut self, velocity: Vector2) {
        if velocity.x > 0. {
            self.set_animation("right_walk".into());
            self.animation_state.replace("right_walk".into());
        } else if velocity.x < 0. {
            self.set_animation("left_walk".into());
            self.animation_state.replace("left_walk".into());
        }

        if velocity.y < 0. {
            self.set_animation("back_walk".into());
            self.animation_state.replace("back_walk".into());
        } else if velocity.y > 0. {
            self.set_animation("front_walk".into());
            self.animation_state.replace("front_walk".into());
        }

        self.play_animation();
    }

    #[func]
    fn play_idle_animation(&mut self) {
        let animation_state = self.animation_state.borrow().clone();
        if let Some(idle_animation) = self.idle_animation_map.get(&animation_state) {
            self.set_animation(idle_animation.clone());
            self.play_animation();
        };
    }

    #[func]
    fn walk(&mut self, velocity: Vector2) {
        let position = self.base_mut().get_position();
        let transition = position + velocity;

        self.base_mut().set_position(transition);
        self.base_mut().move_and_slide();

        if velocity != Vector2::ZERO {
            self.play_walk_animation(velocity);
        } else {
            self.play_idle_animation();
        }
    }

    #[func]
    fn walk_controller(&mut self) {
        let keys = Rc::clone(&self.keys);
        let key_state = &self.key_state;

        // Stoping logic
        keys.iter().for_each(|k| {
            let state = match key_state.borrow_mut().get(k) {
                Some(state) => *state,
                None => return,
            };

            if !state {
                match *k {
                    Key::W => self.direction.y = 0.,
                    Key::D => self.direction.x = 0.,
                    Key::S => self.direction.y = 0.,
                    Key::A => self.direction.x = 0.,
                    _ => {}
                };
            }
        });

        // Moving logic
        let count = key_state.borrow().iter().filter(|(_, &v)| v).count();

        keys.iter().for_each(|k| {
            let state = match key_state.borrow_mut().get(k) {
                Some(state) => *state,
                None => return,
            };

            if state {
                match *k {
                    Key::W => self.direction.y = -1.,
                    Key::D => self.direction.x = 1.,
                    Key::S => self.direction.y = 1.,
                    Key::A => self.direction.x = -1.,
                    _ => {}
                };

                if count > 1 {
                    self.direction = self.direction.normalized();
                }
            }
        });
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 180.0,
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
            idle_animation_map: Rc::new(HashMap::from([
                ("front_walk".into(), "front_idle".into()),
                ("back_walk".into(), "back_idle".into()),
                ("left_walk".into(), "left_idle".into()),
                ("right_walk".into(), "right_idle".into()),
            ])),
            animation_state: Rc::new(RefCell::new("front_idle".into())),
        }
    }

    fn ready(&mut self) {
        let init_animation = self.animation_state.borrow().clone();

        self.init_animation_node();
        self.set_animation(init_animation);
    }

    fn physics_process(&mut self, delta: f64) {
        self.walk_controller();
        self.walk(self.direction * self.speed * delta as f32);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let keys = Rc::clone(&self.keys);
        let key_state = &self.key_state;

        if let Ok(e) = event.try_cast::<InputEventKey>() {
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
    }
}
