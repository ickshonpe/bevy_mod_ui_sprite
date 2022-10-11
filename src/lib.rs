use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::ui::RenderUiSystem;

#[derive(Clone, Component, Copy, Debug, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct UiSprite(pub Vec2);

#[derive(Bundle, Default)]
pub struct UiSpriteBundle {
    pub sprite: UiSprite,    
    pub color: UiColor,
    pub image: UiImage,
    pub visibility: Visibility,
    pub computed: ComputedVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

fn extract_texture_atlas_image_uinodes(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    rect_query: Extract<
        Query<(
            &UiSprite,
            &GlobalTransform,
            &UiColor,
            &UiImage,
            &ComputedVisibility,
        )>,
    >,
) {
    for (&sprite, transform, color, image, visibility) in rect_query.iter() {
        if !visibility.is_visible() {
            continue;
        }
        let rect = bevy::sprite::Rect { min: Vec2::ZERO, max: *sprite };
        extracted_uinodes.uinodes.push(ExtractedUiNode {
            transform: transform.compute_matrix(),
            color: color.0,
            rect,
            image: image.0.clone(),
            atlas_size: None,
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