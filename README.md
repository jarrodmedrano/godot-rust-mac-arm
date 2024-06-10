# Rust and Godot and Mac, living together in peace

I built this starter repo, I was having trouble getting it to work by following other people's tutorials. Hopefully this helps someone.

This repo is specifically for people who use Macs with the M1 chip.

The key was adding the .cargo/config.toml and specifying that you're building for the M1 chip.

```target-dir = "target"
target = "aarch64-apple-darwin"
```

You also need to make sure the file godot/rust.gdextension is connecting to the correct built file:

```
[libraries]
macos.debug.arm64 = "res://../target/aarch64-apple-darwin/debug/librust.dylib"
macos.release.arm64 = "res://../target/aarch64-apple-darwin/debug/librust.dylib"
```

Here I am building to a target folder in the root directory. So it needs to go up one level ../target.

## how to build

```
cd rust
cargo build
```

Once the build is done, open project.godot and voila!

Learn more about the [Rust and Godot book](https://godot-rust.github.io/book/).
