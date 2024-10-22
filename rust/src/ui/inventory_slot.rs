use godot::{
    classes::{Button, IVBoxContainer, Label, MenuButton, Texture2D, TextureRect, VBoxContainer},
    prelude::*,
};

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum SlotType {
    RightHand,
    LeftHand,
    Potions,
    NotEquippable,
}

// #[godot_api]
// impl InventorySlot {
//     #[signal]
//     fn on_popup_menu_item_pressed(&mut self);
// }

#[derive(GodotClass)]
#[class(tool, init, base=VBoxContainer)]
pub struct InventorySlot {
    #[export]
    #[init(val = true)]
    is_empty: bool,
    #[export]
    #[init(val = false)]
    is_selected: bool,
    #[export]
    #[init(val = false)]
    single_button_press: bool,
    #[export]
    starting_texture: Option<Gd<Texture2D>>,
    #[export]
    starting_label: GString,
    #[init(node = "NinePatchRect/MenuButton/CenterContainer/TextureRect")]
    texture_rect: OnReady<Gd<TextureRect>>,
    #[init(node = "NameLabel")]
    name_label: OnReady<Gd<Label>>,
    #[init(node = "NinePatchRect/StackLabel")]
    stack_label: OnReady<Gd<Label>>,
    #[init(node = "PriceLabel")]
    price_label: OnReady<Gd<Label>>,
    #[init(node = "NinePatchRect/OnClickButton")]
    on_click_buttion: OnReady<Gd<Button>>,
    #[init(node = "NinePatchRect/MenuButton")]
    menu_button: OnReady<Gd<MenuButton>>,
    #[init(val = SlotType::NotEquippable)]
    slot_to_equip: SlotType,
    base: Base<VBoxContainer>,
}

#[godot_api]
impl IVBoxContainer for InventorySlot {
    fn ready(&mut self) {
        if let Some(starting_texture) = self.starting_texture.clone() {
            self.texture_rect.set_texture(starting_texture);
        }

        if !self.starting_label.is_empty() {
            self.name_label.set_text(self.starting_label.clone());
        }

        self.menu_button.set_disabled(self.single_button_press);
        self.on_click_buttion
            .set_disabled(!self.single_button_press);
        self.on_click_buttion.set_visible(self.single_button_press);
    }
}
