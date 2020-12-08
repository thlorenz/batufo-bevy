use bevy::prelude::*;

use crate::ecs::components::{Hero, Velocity};

#[derive(Default)]
pub struct PlayerMovement;

impl Plugin for PlayerMovement {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(player_movement_system);
        // .add_system(player_velocity_damp_system);
    }
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Hero>>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0;
        // println!("player: {}", transform.translation);
    }
}

fn player_velocity_damp_system(time: Res<Time>, mut query: Query<&mut Velocity, With<Hero>>) {
    let damp = 5.0_f32;
    let dt = time.delta_seconds();
    for mut velocity in query.iter_mut() {
        if velocity.0.x < 0.0 {
            velocity.0.x += damp * dt
        } else if velocity.0.x > 0.0 {
            velocity.0.x -= damp * dt
        }
    }
}
