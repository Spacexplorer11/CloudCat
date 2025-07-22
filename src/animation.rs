use crate::{AnimationIndices, AnimationTimer};
use bevy::prelude::{Query, Res, Sprite, Time};

pub mod animation {
    use crate::{AnimationIndices, AnimationTimer};
    use bevy::asset::{AssetServer, Assets};
    use bevy::image::{TextureAtlas, TextureAtlasLayout};
    use bevy::math::{UVec2, Vec3};
    use bevy::prelude::{Commands, Res, ResMut, Transform};
    use bevy::sprite::Sprite;
    use bevy::time::{Timer, TimerMode};
    pub(crate) fn animate(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        filepath: &str,
        frames: usize,
        fps: f32,          // frames per second
        translation: Vec3, // position (x, y)
    ) {
        let texture = asset_server.load(filepath);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), frames as u32, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices {
            first: 0,
            last: frames - 1,
        };
        commands.spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            Transform {
                scale: Vec3::splat(6.0),
                translation,
                ..Default::default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(fps, TimerMode::Repeating)),
        ));
    }
}

pub(crate) fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
