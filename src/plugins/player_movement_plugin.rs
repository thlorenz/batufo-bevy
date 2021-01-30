use bevy::prelude::*;

use crate::ecs::components::{Hero, Velocity};
use crate::ecs::resources::TileState;
use crate::engine::WorldPosition;
use crate::plugins::game_plugin::GameRender;

#[derive(Default)]
pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(apply_velocity.system())
            .add_system(update_player_tile.system());
    }
}

// TODO(thlorenz): separate plugin and needs dt
fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0;
    }
}

fn update_player_tile(
    game_render: Res<GameRender>,
    mut tile_state: ResMut<TileState>,
    query: Query<&Transform, With<Hero>>,
) {
    if let Some(transform) = query.iter().next() {
        tile_state.hero_tile =
            WorldPosition::from(&transform.translation).to_tile_position(game_render.tile_size);
    }
}

fn _player_roll_system(mut query: Query<(&mut Transform, &Velocity), With<Hero>>) {
    let max_rot = f32::to_radians(30.0);
    let rot_factor = 0.1;
    for (mut transform, velocity) in query.iter_mut() {
        let rot = if velocity.0.x > 0.0 {
            (-velocity.0.x * rot_factor).max(-max_rot)
        } else {
            (-velocity.0.x * rot_factor).min(max_rot)
        };
        let current_rot = transform.rotation.z;
        transform.rotate(Quat::from_rotation_z(rot - current_rot));
    }
}

fn _player_velocity_damp_system(time: Res<Time>, mut query: Query<&mut Velocity, With<Hero>>) {
    let damp = 6.0_f32;
    let dt = time.delta_seconds();
    for mut velocity in query.iter_mut() {
        if velocity.0.x < 0.0 {
            velocity.0.x += damp * dt
        } else if velocity.0.x > 0.0 {
            velocity.0.x -= damp * dt
        }
    }
}
