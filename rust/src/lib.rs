pub mod item;
pub mod pick_up_item;
pub mod player;
pub mod slot_type;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
