pub mod inventory;
pub mod item;
pub mod pick_up_item;
pub mod player;
pub mod ui;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
