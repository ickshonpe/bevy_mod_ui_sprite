use bevy::{math::vec2, window::PresentMode};
use bevy::prelude::*;
use bevy_despawn_with::DespawnAllCommandsExt;
use bevy_mod_ui_sprite::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

const SPRITE_PATHS: [&str; 2] = ["sprite.png", "bevy.png"];

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_sprites( 
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.despawn_all::<With<UiSprite>>();
    for x in 0..100 {
        for y in 0..100 {
            let target = 16. * vec2(x as f32, y as f32) - vec2(500., 250.);
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite { 
                    color: Color::RED,
                    ..Default::default()
                },
                transform: Transform::from_translation(target.extend(0.0)),
                texture: asset_server.load(SPRITE_PATHS[(x + y) % 2]),
                ..Default::default()
            });
        }
    }
}

fn spawn_ui_sprites( 
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.despawn_all::<With<Sprite>>();
    for x in 0..100 {
        for y in 0..100 {
            let target = 16. * vec2(x as f32, y as f32) - vec2(500., 250.);
            commands.spawn_bundle(UiSpriteBundle {
                sprite: UiSprite::Image(asset_server.load(SPRITE_PATHS[(x + y) % 2])),     
                color: UiColor(Color::GREEN),
                transform: Transform::from_translation(target.extend(0.0)),
                ..Default::default()
            });
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum SpritesState {
    Sprites,
    UiSprites,
}

pub fn swap_state(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<SpritesState>>
) {
    if input.just_pressed(KeyCode::Space) {
        let next_state = match state.current() {
            SpritesState::Sprites => SpritesState::UiSprites,
            SpritesState::UiSprites => SpritesState::Sprites
        };
        println!("state => {:?}", next_state);
        let _ = state.set(next_state);
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1000.,
            height: 500.,
            scale_factor_override: Some(2.0),
            present_mode: PresentMode::Immediate,
            ..Default::default()
        })
        .add_state(SpritesState::UiSprites)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(UiSpritePlugin)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::on_enter(SpritesState::Sprites)
            .with_system(spawn_sprites)
        )
        .add_system_set(
            SystemSet::on_enter(SpritesState::UiSprites)
            .with_system(spawn_ui_sprites)
        )
        .add_system(swap_state)
        .run();
}
