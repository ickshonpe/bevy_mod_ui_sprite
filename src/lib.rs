use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::sprite::Rect;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::ui::RenderUiSystem;

/// The texture of the sprite.
/// Use UiSprite::default() for a flat textureless color.
#[derive(Clone, Component, Debug, Reflect)]
#[reflect(Component)]
pub enum UiSprite {
    Image(
        /// handle of the texture
        Handle<Image>,
    ),
    AtlasImage {
        /// handle of the texture atlas
        handle: Handle<TextureAtlas>,
        /// index of the texture in the texture atlas
        index: usize,
    }
}

impl Default for UiSprite {
    fn default() -> Self {
        UiSprite::Image(
            // flat textureless color
            DEFAULT_IMAGE_HANDLE.typed(),
        )
    }
}

/// Size of the sprite
#[derive(Copy, Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub enum SpriteSize {
    /// use the sprite's image's size
    #[default]
    Auto,
    /// stretch / shrink the sprite to this size
    Size(Vec2)
}



#[derive(Bundle, Default)]
pub struct UiSpriteBundle {
    /// texture of the sprite
    pub sprite: UiSprite,    
    /// color of the sprite
    pub color: UiColor,
    /// size of the sprite
    pub size: SpriteSize,
    pub visibility: Visibility,
    pub computed: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[allow(clippy::type_complexity)]
fn extract_texture_atlas_image_uinodes(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    images: Extract<Res<Assets<Image>>>,
    texture_atlases: Extract<Res<Assets<TextureAtlas>>>,
    rect_query: Extract<
        Query<(
            &UiSprite,
            &GlobalTransform,
            &UiColor,
            &SpriteSize,
            &ComputedVisibility,
        )>,
    >,
) {
    for (sprite, transform, color, size, visibility) in rect_query.iter() {
        if !visibility.is_visible() {
            continue;
        }
        let mut transform = transform.compute_matrix();
        let (image, rect, atlas_size) =
            match sprite {
                UiSprite::Image ( handle ) => {
                    if let Some(image) = images.get(handle) {
                        (
                            handle.clone(),    
                            Rect { 
                                min: Vec2::ZERO,
                                max: match *size {
                                    SpriteSize::Size(size) => size,
                                    SpriteSize::Auto => image.size(),
                                }
                            },
                            None
                        )
                    } else {
                        continue;
                    }
                },
                UiSprite::AtlasImage { handle, index } => {
                    if let Some(atlas) = texture_atlases.get(handle) {
                        if !images.contains(&atlas.texture) {
                            continue;
                        }
                        let rect = atlas.textures[*index];
                        if let &SpriteSize::Size(size) = size {
                            let scale = size / rect.size();
                            transform *= Mat4::from_scale(scale.extend(1.0));
                        }
                        (
                            atlas.texture.clone(),
                            rect,
                            Some(atlas.size)
                        )
                    } else {
                        continue;
                    }
                },
            };
        extracted_uinodes.uinodes.push(ExtractedUiNode {
            transform,
            color: color.0,
            rect,
            image,
            atlas_size,
            clip: None,
        });
    }
}

pub struct UiSpritePlugin;

impl Plugin for UiSpritePlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<UiSprite>();

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app.add_system_to_stage(
            RenderStage::Extract,
            extract_texture_atlas_image_uinodes.after(RenderUiSystem::ExtractNode),
        );
    }
}