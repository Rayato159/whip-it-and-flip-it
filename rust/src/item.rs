use godot::{
    classes::{RectangleShape2D, Resource, Texture2D},
    prelude::*,
};

use crate::ui::inventory_slot::SlotType;

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct Item {
    #[export]
    name: GString,
    #[export]
    price: u32,
    #[export]
    #[init(val = 99)]
    max_stacks: i64,
    #[export]
    collision_shape: Option<Gd<RectangleShape2D>>,
    #[export]
    texture: Option<Gd<Texture2D>>,
    #[export]
    side_texture: Option<Gd<Texture2D>>,
    #[export]
    #[init(val = 1)]
    stacks: i64,
    #[export]
    #[init(val = SlotType::NotEquippable)]
    slot_type: SlotType,
    base: Base<Resource>,
}
