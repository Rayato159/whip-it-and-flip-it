use godot::{
    classes::{InputEvent, InputEventKey},
    global::Key,
    prelude::*,
};

use crate::{
    item::{self, Item},
    pick_up_item::PickUpItem,
    ui::inventory_ui::InventoryUI,
};

#[derive(GodotClass)]
#[class(tool, init, base=Node)]
pub struct Inventory {
    #[init(node = "../InventoryUI")]
    inventory_ui: OnReady<Gd<InventoryUI>>,
    #[export]
    #[init(val = array![])]
    items: Array<Option<Gd<Item>>>,
    base: Base<Node>,
}

#[godot_api]
impl Inventory {
    #[signal]
    fn on_toggle(&mut self);

    #[signal]
    fn on_add_item(&mut self, item_gd: Gd<Item>);

    #[func]
    fn add_item(&mut self, item_gd: Gd<Item>, stacks: i64) {
        if stacks > 0 && item_gd.bind().get_max_stacks() > 1 {
            self.add_stackable_item_into_inventory(item_gd.clone(), stacks);
        } else {
            self.items.extend_array(&array![Some(item_gd.clone())]);

            self.base_mut()
                .emit_signal("on_add_item".into(), &[item_gd.clone().to_variant()]);
        }
    }

    #[func]
    fn add_stackable_item_into_inventory(&mut self, item_gd: Gd<Item>, stacks: i64) {
        let mut item_index: Option<usize> = None;

        // Reverse searching for the item in the inventory
        for i in 0..self.items.len() {
            if let Some(inventory_item) = self.items.at(i) {
                if inventory_item.bind().get_name() == item_gd.bind().get_name() {
                    item_index = Some(i);
                };
            };
        }

        // If the item is found in the inventory
        if !item_index.is_none() {
            if let Some(mut inventory_item_gd) = self.items.at(item_index.unwrap()) {
                if inventory_item_gd.bind().get_stacks() + stacks <= item_gd.bind().get_max_stacks()
                {
                    let new_stacks = inventory_item_gd.bind().get_stacks() + stacks;
                    inventory_item_gd.bind_mut().set_stacks(new_stacks);

                    self.items.set(item_index.unwrap(), Some(inventory_item_gd));
                } else {
                    inventory_item_gd
                        .bind_mut()
                        .set_stacks(item_gd.bind().get_max_stacks());

                    self.items
                        .set(item_index.unwrap(), Some(inventory_item_gd.clone()));

                    if let Some(inventory_item_gd_dub) = inventory_item_gd.duplicate() {
                        let mut new_item_gd =
                            inventory_item_gd_dub.clone().try_cast::<Item>().unwrap();

                        let stacks_diff = inventory_item_gd.bind().get_stacks() + stacks
                            - item_gd.bind().get_max_stacks();

                        new_item_gd.bind_mut().set_stacks(stacks_diff);

                        self.items
                            .extend(array![Some(new_item_gd.clone())].iter_shared());
                        self.base_mut()
                            .emit_signal("on_add_item".into(), &[new_item_gd.to_variant()]);
                    };
                }
            };
        } else {
            if let Some(new_item_gd_dub) = item_gd.duplicate() {
                let mut new_item_gd = new_item_gd_dub.try_cast::<Item>().unwrap();
                new_item_gd.bind_mut().set_stacks(stacks);

                self.items
                    .extend(array![Some(new_item_gd.clone())].iter_shared());

                self.base_mut()
                    .emit_signal("on_add_item".into(), &[new_item_gd.to_variant()]);
            };
        }
    }
}

#[godot_api]
impl INode for Inventory {
    fn ready(&mut self) {
        let mut pick_up_item_node = self
            .base_mut()
            .get_node_as::<PickUpItem>("../../PickUpItem");
        let add_item_callable = self.base().callable("add_item");
        pick_up_item_node.connect("on_add_item".into(), add_item_callable.clone());

        let mut pick_up_item_node_2 = self
            .base_mut()
            .get_node_as::<PickUpItem>("../../PickUpItem2");
        pick_up_item_node_2.connect("on_add_item".into(), add_item_callable.clone());

        let mut pick_up_item_node_3 = self
            .base_mut()
            .get_node_as::<PickUpItem>("../../PickUpItem3");
        pick_up_item_node_3.connect("on_add_item".into(), add_item_callable);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(e) = event.try_cast::<InputEventKey>() {
            if e.is_pressed() {
                if e.get_keycode() == Key::TAB {
                    self.base_mut().emit_signal("on_toggle".into(), &[]);
                }
            }
        }
    }
}
