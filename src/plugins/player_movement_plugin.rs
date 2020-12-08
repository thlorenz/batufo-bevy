use bevy::prelude::*;

use crate::ecs::components::{Hero, Velocity};

#[derive(Default)]
pub struct PlayerMovement;

impl Plugin for PlayerMovement {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(player_movement_system)
            .add_system(player_roll_system)
            .add_system(player_velocity_damp_system);
    }
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Hero>>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0;
    }
}

fn player_roll_system(mut query: Query<(&mut Transform, &Velocity), With<Hero>>) {
    let max_rot = f32::to_radians(30.0);
    let rot_factor = 0.1;
    for (mut transform, velocity) in query.iter_mut() {
        let rot = if velocity.0.x > 0.0 {
            (-velocity.0.x * rot_factor).max(-max_rot)
        } else {
            (-velocity.0.x * rot_factor).min(max_rot)
        };
        transform.rotation = Quat::from_rotation_z(rot);
    }
}

fn player_velocity_damp_system(time: Res<Time>, mut query: Query<&mut Velocity, With<Hero>>) {
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
