# bevy_mod_ui_sprite
[![crates.io](https://img.shields.io/crates/v/bevy_mod_ui_sprite)](https://crates.io/crates/bevy_mod_ui_sprite)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_mod_ui_sprite)
[![crates.io](https://img.shields.io/crates/d/bevy_mod_ui_sprite)](https://crates.io/crates/bevy_mod_ui_sprite)

## usage

Add the dependency to `Cargo.toml`:

```toml
bevy_mod_ui_sprite = "0.1.2"
```

Add the plugin to your app:

```rust
use bevy_mod_ui_sprite::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiSpritePlugin)
        // ..rest of app
        .run()
}
```
You also need a camera:

```rust
commands.spawn_bundle(Camera2dBundle::default());
```

Then you can spawn a UiSpriteBundle:

```rust
 commands.spawn_bundle(UiSpriteBundle {
    sprite: UiSprite(Vec2::splat(100.)),
    transform: Transform::from_translation(Vec3::splat(100.)),
    color: UiColor(Color::RED),
    ..Default::default()
});
```

## Full Example

```
cargo --run --example example
```
