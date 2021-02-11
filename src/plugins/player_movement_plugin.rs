use std::f32::EPSILON;

use bevy::prelude::*;

use crate::{
    ecs::components::{Hero, HeroHull, Velocity},
    engine::physics::perp_vector_for_rotation_y,
};

#[derive(Default)]
pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(apply_velocity.system())
            .add_system(player_roll.system());
    }
}

// TODO(thlorenz): separate plugin and needs dt
fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0;
    }
}

fn rot_for_perp_velocity(perp_vel: f32) -> f32 {
    let max_rot = f32::to_radians(30.0);
    let rot_factor = 1.5;
    if perp_vel > EPSILON {
        (-perp_vel * rot_factor).max(-max_rot)
    } else if perp_vel < EPSILON {
        (-perp_vel * rot_factor).min(max_rot)
    } else {
        0.0
    }
}

fn player_roll(
    hero_query: Query<(&Velocity, &Transform), With<Hero>>,
    mut hull_query: Query<&mut Transform, With<HeroHull>>,
) {
    if let Some((velocity, Transform { rotation, .. })) = hero_query.iter().next() {
        if let Some(mut hull_transform) = hull_query.iter_mut().next() {
            let current_rot = hull_transform.rotation.z;
            let Vec3 { x, z, .. } = perp_vector_for_rotation_y(rotation);
            if z.abs() > x.abs() {
                // facing along z-axis -> rotate around x-axis
                let rot = rot_for_perp_velocity(velocity.0.z * z);
                hull_transform.rotate(Quat::from_rotation_z(rot - current_rot));
            } else {
                // facing along x-axis -> rotate around z-axis
                let rot = rot_for_perp_velocity(velocity.0.x * x);
                hull_transform.rotate(Quat::from_rotation_z(rot - current_rot));
            }
        }
    }
}
