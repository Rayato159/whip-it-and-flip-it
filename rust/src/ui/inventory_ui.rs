use godot::{
    classes::{CanvasLayer, GridContainer, ICanvasLayer},
    prelude::*,
};

use crate::{inventory::Inventory, item::Item};

use super::inventory_slot::{InventorySlot, SlotType};

#[derive(GodotClass)]
#[class(tool, init, base=CanvasLayer)]
pub struct InventoryUI {
    #[init(node = "./MarginContainer/NinePatchRect/MarginContainer/VBoxContainer/GridContainer")]
    grid_container: OnReady<Gd<GridContainer>>,
    inventory_node: Option<Gd<Inventory>>,
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

    #[func]
    fn add_item(&mut self, item_gd: Gd<Item>) {
        let empty_slot_index = self.get_empty_slot();
        if let Ok(mut slot_gd) = self
            .grid_container
            .get_children()
            .at(empty_slot_index as usize)
            .try_cast::<InventorySlot>()
        {
            if item_gd.bind().get_slot_type() != SlotType::NotEquippable.to_gd_string() {
                let equip_slot_text =
                    self.get_equipable_popup_text(item_gd.bind().get_slot_type().to_string());

                slot_gd
                    .bind_mut()
                    .get_menu_button()
                    .set_text(equip_slot_text);
            }

            slot_gd.bind_mut().set_is_empty(false);

            let mut menu_button_context = slot_gd.bind().get_menu_button().clone();
            menu_button_context.set_disabled(false);
            slot_gd.bind_mut().set_menu_button(menu_button_context);

            let mut name_label = slot_gd.bind().get_name_label().clone();
            name_label.set_text(item_gd.bind().get_name());
            slot_gd.bind_mut().set_name_label(name_label);

            if let Some(item_texture) = item_gd.bind().get_texture() {
                let mut new_texture_rect = slot_gd.bind().get_texture_rect().clone();
                new_texture_rect.set_texture(item_texture);

                slot_gd.bind_mut().set_texture_rect(new_texture_rect);
            }

            if item_gd.bind().get_stacks() < 2 {
                return;
            }
            let mut stack_label = slot_gd.bind().get_stack_label().clone();
            stack_label.set_text(item_gd.bind().get_stacks().to_string().into());
            slot_gd.bind_mut().set_stack_label(stack_label);
        }
    }

    #[func]
    fn get_empty_slot(&mut self) -> i32 {
        for slot_index in 0..self.grid_container.get_children().len() - 1 {
            let slot_node = self.grid_container.get_children().at(slot_index);
            if let Ok(s) = slot_node.try_cast::<InventorySlot>() {
                if s.bind().get_is_empty() {
                    return slot_index as i32;
                }
            }
        }
        -1
    }

    #[func]
    fn get_equipable_popup_text(&self, value: String) -> GString {
        match value.as_str() {
            "RightHand" => "Equip to Right Hand".into(),
            "LeftHand" => "Equip to Left Hand".into(),
            _ => value.into(),
        }
    }
}

#[godot_api]
impl ICanvasLayer for InventoryUI {
    fn ready(&mut self) {
        let mut inventory_node = self.base_mut().get_node_as::<Inventory>("../Inventory");
        self.inventory_node = Some(inventory_node.clone());
        let toggle_callable = self.base().callable("toggle");
        let add_item_callable = self.base().callable("add_item");
        inventory_node.connect("on_toggle".into(), toggle_callable);
        inventory_node.connect("on_add_item".into(), add_item_callable);

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
