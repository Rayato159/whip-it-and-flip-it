use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use godot::classes::{
    AnimatedSprite2D, Area2D, CanvasLayer, CharacterBody2D, CollisionShape2D, ICharacterBody2D,
    InputEvent, InputEventKey,
};
use godot::global::Key;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    speed: f32,
    direction: Vector2,
    walk_key_state: Rc<RefCell<HashMap<Key, bool>>>,
    walk_keys: Rc<Vec<Key>>,
    is_invetory_open: bool,
    base: Base<CharacterBody2D>,
    animation_node: Rc<RefCell<Option<Gd<AnimatedSprite2D>>>>,
    collision_shape2d_node: Rc<Option<Gd<CollisionShape2D>>>,
    area2d: Rc<Option<Gd<Area2D>>>,
    idle_animation_map: Rc<HashMap<StringName, StringName>>,
    animation_state: Rc<RefCell<StringName>>,
}

#[godot_api]
impl Player {
    #[signal]
    fn on_area2d_entered(&mut self);

    #[func]
    fn set_animation(&mut self, animation: StringName) {
        let animation_node = Rc::clone(&self.animation_node);
        let animation_node_borrow_mut = &mut *animation_node.borrow_mut();

        if let Some(node) = animation_node_borrow_mut {
            node.set_animation(animation)
        };
    }

    #[func]
    fn play_animation(&mut self) {
        let animation_node = Rc::clone(&self.animation_node);
        let animation_node_borrow_mut = &mut *animation_node.borrow_mut();

        if let Some(node) = animation_node_borrow_mut {
            node.play();
        };
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
        let animation_state = Rc::clone(&self.animation_state);
        let idle_animation_map = Rc::clone(&self.idle_animation_map);

        let state = animation_state.borrow_mut();

        if let Some(s) = idle_animation_map.get(&state) {
            self.set_animation(s.clone());
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
        let walk_keys = Rc::clone(&self.walk_keys);
        let walk_key_state = Rc::clone(&self.walk_key_state);

        // Stoping logic
        walk_keys.iter().for_each(|k| {
            let state = match walk_key_state.borrow_mut().get(k) {
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
        let count = walk_key_state
            .borrow_mut()
            .iter()
            .filter(|(_, &v)| v)
            .count();

        walk_keys.iter().for_each(|k| {
            let state = match walk_key_state.borrow_mut().get(k) {
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
            walk_key_state: Rc::new(RefCell::new(HashMap::from([
                (Key::W, false),
                (Key::D, false),
                (Key::S, false),
                (Key::A, false),
            ]))),
            walk_keys: Rc::new(vec![Key::W, Key::D, Key::S, Key::A]),
            is_invetory_open: false,
            base,
            animation_node: Rc::new(RefCell::new(None)),
            collision_shape2d_node: Rc::new(None),
            area2d: Rc::new(None),
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
        let collision_shape_node = self
            .base_mut()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        self.collision_shape2d_node = Rc::new(Some(collision_shape_node));

        let animation_node = self
            .base_mut()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        self.animation_node = Rc::new(RefCell::new(Some(animation_node)));

        let init_animation = Rc::clone(&self.animation_state);
        self.set_animation(init_animation.borrow_mut().clone());

        let area2d_node = self.base_mut().get_node_as::<Area2D>("Area2D");
        let area = area2d_node.cast::<Area2D>();
        self.area2d = Rc::new(Some(area));
    }

    fn physics_process(&mut self, delta: f64) {
        let area2d = Rc::clone(&self.area2d);
        self.base_mut()
            .emit_signal("on_area2d_entered".into(), &[area2d.to_variant()]);

        let mut inventory_ui_node = self.base_mut().get_node_as::<CanvasLayer>("InventoryUI");
        let is_invetory_open = self.is_invetory_open;
        inventory_ui_node.set_visible(is_invetory_open);
        if !is_invetory_open {
            self.walk_controller();
            self.walk(self.direction * self.speed * delta as f32);
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let walk_keys = Rc::clone(&self.walk_keys);
        let walk_key_state = &self.walk_key_state;

        if let Ok(e) = event.try_cast::<InputEventKey>() {
            if e.is_pressed() {
                walk_keys.iter().for_each(|k| {
                    if e.get_keycode() == *k {
                        walk_key_state.borrow_mut().insert(*k, true);
                    }
                });

                if e.get_keycode() == Key::TAB {
                    self.is_invetory_open = !self.is_invetory_open;
                }
            }

            if e.is_released() {
                walk_keys.iter().for_each(|k| {
                    if e.get_keycode() == *k {
                        walk_key_state.borrow_mut().insert(*k, false);
                    }
                });
            }
        }
    }
}
