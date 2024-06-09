mod player;

use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {
    fn name() -> &'static str {
        "RustExtension"
    }
}