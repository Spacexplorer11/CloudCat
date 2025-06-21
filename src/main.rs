mod animation;

use crate::animation::animate_sprite;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, game_loop)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn game_loop(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let cat_walk_speed: f32 = 0.1;
    commands.spawn(Camera2d);
    animation::animation::animate(
        commands,
        asset_server,
        texture_atlas_layouts,
        "cat.png",
        3,
        cat_walk_speed,
    );
}
