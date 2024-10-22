use godot::{
    classes::{CanvasLayer, GridContainer, ICanvasLayer},
    prelude::*,
};

use crate::inventory::Inventory;

#[derive(GodotClass)]
#[class(tool, init, base=CanvasLayer)]
pub struct InventoryUI {
    #[init(node = "./MarginContainer/NinePatchRect/MarginContainer/VBoxContainer/GridContainer")]
    grid_container: OnReady<Gd<GridContainer>>,
    #[export]
    #[init(val = 8)]
    size: i64,
    #[export]
    #[init(val = 4)]
    columns: i64,
    base: Base<CanvasLayer>,
}

#[godot_api]
impl InventoryUI {
    #[func]
    fn toggle(&mut self) {
        let is_visible = self.base().is_visible();
        self.base_mut().set_visible(!is_visible);
    }
}

#[godot_api]
impl ICanvasLayer for InventoryUI {
    fn ready(&mut self) {
        let mut inventory_node = self.base_mut().get_node_as::<Inventory>("../Inventory");
        let toggle_callable = self.base().callable("toggle");
        inventory_node.connect("on_toggle".into(), toggle_callable);

        self.grid_container.set_columns(self.columns as i32);

        for _ in 0..self.size {
            let inventory_slot_scene =
                match load::<PackedScene>("res://Scenes/UI/inventory_slot.tscn").instantiate() {
                    Some(scene) => scene,
                    None => {
                        godot_error!("Failed to load inventory slot scene");
                        return;
                    }
                };

            self.grid_container.add_child(inventory_slot_scene.clone());
        }
    }
}
