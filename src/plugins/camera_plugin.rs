use bevy::prelude::*;

use crate::arena::arena::Arena;

use super::game_plugin::GameProps;

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_camera);
    }
}

fn setup_camera(commands: &mut Commands, game_props: Res<GameProps>, arena: Res<Arena>) {
    let player_pos = arena.player.to_world_position(game_props.render.tile_size);
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(player_pos.x, player_pos.y, 0.0)),
        ..Default::default()
    });
}
