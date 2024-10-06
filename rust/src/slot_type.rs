use godot::prelude::*;

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum SlotType {
    RightHand,
    LeftHand,
    Potions,
    NotEquippable,
}
