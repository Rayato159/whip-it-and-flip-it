use godot::{
    classes::{InputEvent, InputEventKey},
    global::Key,
    prelude::*,
};

use crate::ui::inventory_ui::InventoryUI;

#[derive(GodotClass)]
#[class(tool, init, base=Node)]
pub struct Inventory {
    #[init(node = "../InventoryUI")]
    inventory_ui: OnReady<Gd<InventoryUI>>,
    base: Base<Node>,
}

#[godot_api]
impl Inventory {
    #[signal]
    fn on_toggle(&mut self);
}

#[godot_api]
impl INode for Inventory {
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
