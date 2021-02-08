use bevy::prelude::*;
use plugins::{LifeCyclePlugin, ProjectilePlugin};

use crate::plugins::{
    ArenaPlugin, CameraPlugin, GamePlugin, GunTowerPlugin, LightPlugin, PlayerInputPlugin,
    PlayerMovementPlugin, PlayerRenderPlugin,
};

mod ai;
mod animations;
mod arena;
mod ecs;
mod engine;
mod plugins;
mod utils;

const BG: &str = "000000"; // "1E1C32"

fn main() {
    App::build()
        .add_resource(ClearColor(
            Color::hex(BG).expect("Invalid Background Color"),
        ))
        .add_plugin(GamePlugin)
        .add_plugin(LightPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(ArenaPlugin)
        .add_plugin(LifeCyclePlugin)
        .add_plugin(PlayerRenderPlugin)
        .add_plugin(PlayerInputPlugin)
        .add_plugin(PlayerMovementPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(GunTowerPlugin)
        .run();
}
