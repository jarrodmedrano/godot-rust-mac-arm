use godot::prelude::*;
mod player;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}