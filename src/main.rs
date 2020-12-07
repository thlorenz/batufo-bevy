mod arena;
mod engine;
mod plugins;

use bevy::prelude::*;
use plugins::{arena_plugin::ArenaPlugin, camera_plugin::CameraPlugin, game_plugin::GamePlugin};

fn main() {
    App::build()
        .add_resource(ClearColor(
            Color::hex("1E1C32").expect("Invalid Background Color"),
        ))
        .add_plugin(GamePlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(ArenaPlugin)
        .run();
}

/*
fn setup(commands: &mut Commands) {
    let texture_handle = asset_server.load("textures/bg/floor-tiles.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 8, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            sprite: TextureAtlasSprite::new(1),
            ..Default::default()
        })
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            sprite: TextureAtlasSprite::new(1),
            ..Default::default()
        });
}
*/
