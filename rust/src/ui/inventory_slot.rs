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

impl SlotType {
    pub fn to_gd_string(&self) -> GString {
        match self {
            SlotType::RightHand => "RightHand".into(),
            SlotType::LeftHand => "LeftHand".into(),
            SlotType::Potions => "Potions".into(),
            SlotType::NotEquippable => "NotEquippable".into(),
        }
    }
}

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
    #[var]
    #[init(node = "NinePatchRect/MenuButton/CenterContainer/TextureRect")]
    texture_rect: OnReady<Gd<TextureRect>>,
    #[var]
    #[init(node = "NameLabel")]
    name_label: OnReady<Gd<Label>>,
    #[var]
    #[init(node = "NinePatchRect/StackLabel")]
    stack_label: OnReady<Gd<Label>>,
    #[var]
    #[init(node = "PriceLabel")]
    price_label: OnReady<Gd<Label>>,
    #[var]
    #[init(node = "NinePatchRect/OnClickButton")]
    on_click_button: OnReady<Gd<Button>>,
    #[var]
    #[init(node = "NinePatchRect/MenuButton")]
    menu_button: OnReady<Gd<MenuButton>>,
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
        self.on_click_button.set_disabled(!self.single_button_press);
        self.on_click_button.set_visible(self.single_button_press);
    }
}
