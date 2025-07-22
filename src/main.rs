mod animation;

use crate::animation::animate_sprite;
use bevy::app::PluginGroup;
use bevy::image::TextureAtlasLayout;
use bevy::math::Vec3;
use bevy::prelude::{Camera2d, Deref, DerefMut, ImagePlugin};
use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    asset::{AssetServer, Assets},
    ecs::{
        component::Component,
        system::{Commands, Res, ResMut},
    },
    time::Timer,
};

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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let cat_walk_speed: f32 = 0.4; // will decrease to go faster as game progresses
    commands.spawn(Camera2d);
    animation::animation::animate(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        "cat.png",
        3,
        cat_walk_speed,
        Vec3::new(-100.0, 0.0, 0.0),
    );
    animation::animation::animate(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        "cloud.png",
        7,
        0.1,
        Vec3::new(150.0, 100.0, 0.0),
    )
}
