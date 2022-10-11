use bevy::prelude::*;
use bevy_mod_ui_sprite::*;

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(UiSpriteBundle {
        sprite: UiSprite(Vec2::splat(100.)),
        transform: Transform::from_translation(Vec3::splat(100.)),
        color: UiColor(Color::RED),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiSpritePlugin)
        .add_startup_system(setup)
        .run();
}
