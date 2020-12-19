use bevy::prelude::*;

use crate::plugins::{
    ArenaPlugin, CameraPlugin, GamePlugin, GunTowerPlugin, LightPlugin, PathFinderPlugin,
    PlayerInputPlugin, PlayerMovementPlugin, PlayerRenderPlugin, TileInteractionPlugin,
};

mod animations;
mod arena;
mod ecs;
mod engine;
mod plugins;

const BG: &str = "000000"; // "1E1C32"

fn main() {
    App::build()
        .add_resource(ClearColor(
            Color::hex(BG).expect("Invalid Background Color"),
        ))
        .add_plugin(TileInteractionPlugin)
        .add_plugin(PathFinderPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(LightPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(ArenaPlugin)
        .add_plugin(PlayerRenderPlugin)
        .add_plugin(PlayerInputPlugin)
        .add_plugin(PlayerMovementPlugin)
        .add_plugin(GunTowerPlugin)
        .run();
}
