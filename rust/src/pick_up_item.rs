use godot::{
    classes::{Area2D, CollisionShape2D, IArea2D, Sprite2D},
    obj::WithBaseField,
    prelude::*,
};

use crate::{inventory::Inventory, item::Item, player::Player};

#[derive(GodotClass)]
#[class(tool, init, base=Area2D)]
pub struct PickUpItem {
    #[export]
    item: Option<Gd<Item>>,
    #[init(node = "../Player/Inventory")]
    inventory: OnReady<Gd<Inventory>>,
    base: Base<Area2D>,
}

#[godot_api]
impl PickUpItem {
    #[signal]
    fn on_add_item(&mut self, item_gd: Gd<Item>, stacks: i64);

    #[func]
    fn area2d_entered(&mut self, player_area2d: Gd<Area2D>) {
        if self.base().overlaps_area(player_area2d) {
            if let Some(item) = self.get_item() {
                self.base_mut().emit_signal(
                    "on_add_item".into(),
                    &[item.to_variant(), item.bind().get_stacks().to_variant()],
                );

                self.base_mut().queue_free();
            };
        }
    }
}

#[godot_api]
impl IArea2D for PickUpItem {
    fn ready(&mut self) {
        let mut player_node = self.base_mut().get_node_as::<Player>("../Player");
        let area2d_entered_callable = self.base().callable("area2d_entered");
        player_node.connect("on_area2d_entered".into(), area2d_entered_callable);

        let item_gd = match self.get_item() {
            Some(item) => item,
            None => return,
        };
        let item = item_gd.bind();

        let mut sprite_node = self.base_mut().get_node_as::<Sprite2D>("Sprite2D");
        if let Some(texture) = item.get_texture() {
            sprite_node.set_texture(texture);
        }

        let mut collision_shape2d_node = self
            .base_mut()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        if let Some(collision_shape) = item.get_collision_shape() {
            collision_shape2d_node.set_shape(collision_shape);
        }
    }
}
