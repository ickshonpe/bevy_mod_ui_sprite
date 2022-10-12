use bevy::prelude::*;
use bevy_mod_ui_sprite::*;

fn setup(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    // colored rectangles
    commands.spawn_bundle(UiSpriteBundle {
        size: SpriteSize::Size(Vec2::splat(100.)),
        transform: Transform::from_translation(Vec3::splat(100.)),
        color: UiColor(Color::RED),
        ..Default::default()
    });
    commands.spawn_bundle(UiSpriteBundle {
        size: SpriteSize::Size(Vec2::splat(100.)),
        transform: Transform::from_translation(Vec3::new(120., 120., 50.)),
        color: UiColor(Color::GREEN),
        ..Default::default()
    });
    commands.spawn_bundle(UiSpriteBundle {
        size: SpriteSize::Size(Vec2::splat(100.)),
        transform: Transform::from_translation(Vec3::splat(140.)),
        color: UiColor(Color::BLUE),
        ..Default::default()
    });

    // textured sprites
    commands.spawn_bundle(UiSpriteBundle {
        sprite: UiSprite::Image(asset_loader.load("sprite.png")),
        transform: Transform::from_translation(Vec3::new(200., 100., 200.)),
        ..Default::default()
    });

    commands.spawn_bundle(UiSpriteBundle {
        sprite: UiSprite::Image(asset_loader.load("sprite.png")),
        size: SpriteSize::Size(Vec2::new(100., 50.)),
        color: UiColor(Color::YELLOW),
        transform: Transform::from_translation(Vec3::new(200., 200., 200.)),
        ..Default::default()
    });

    // sprites from a texture atlas
    let texture_atlas_texture = asset_loader.load("numbered_grid_texture_atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_atlas_texture, Vec2::splat(16.), 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    for (n, index) in [2, 7, 15, 9].into_iter().enumerate() {
        let target = Vec3::new(100. - n as f32 * 8., 50. + n as f32 * 24., 250.);
        commands.spawn_bundle(UiSpriteBundle {
            sprite: UiSprite::AtlasImage { handle: texture_atlas_handle.clone(), index },
            transform: Transform::from_translation(target),
            ..Default::default()
        });
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 300.,
            height: 250.,
            scale_factor_override: Some(2.0),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(UiSpritePlugin)
        .add_startup_system(setup)
        .run();
}
