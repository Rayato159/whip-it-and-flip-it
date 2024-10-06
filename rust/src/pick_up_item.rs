use godot::{
    classes::{Area2D, IArea2D},
    prelude::*,
};

use crate::item::Item;

#[derive(GodotClass)]
#[class(tool, init, base=Area2D)]
pub struct PickUpItem {
    #[export]
    item: Option<Gd<Item>>,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for PickUpItem {
    fn ready(&mut self) {
        // set texture of the item
        // set collision shape of the item
    }
}
