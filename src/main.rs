mod arena;
mod ecs;
mod engine;
mod plugins;

use bevy::prelude::*;
use plugins::{
    arena_plugin::ArenaPlugin, camera_plugin::CameraPlugin, game_plugin::GamePlugin,
    light_plugin::LightPlugin, player_input_plugin::PlayerInput,
    player_movement_plugin::PlayerMovement,
};

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
        .add_plugin(PlayerInput)
        .add_plugin(PlayerMovement)
        .run();
}
