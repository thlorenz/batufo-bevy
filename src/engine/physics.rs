use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};

pub fn angle_for_rotation_y(rotation: &Quat) -> f32 {
    let axis_angle = rotation.to_axis_angle();
    axis_angle.1 * axis_angle.0.y
}

pub fn perpendicular(angle: f32) -> f32 {
    angle + (PI / 2.0)
}

pub fn vector_for_angle_y(angle: f32) -> Vec3 {
    let x = angle.sin();
    let z = angle.cos();
    return Vec3::new(x, 1.0, z).normalize();
}

pub fn vector_for_rotation_y(rotation: &Quat) -> Vec3 {
    let angle = angle_for_rotation_y(rotation);
    vector_for_angle_y(angle)
}

pub fn perp_vector_for_rotation_y(rotation: &Quat) -> Vec3 {
    let angle = perpendicular(angle_for_rotation_y(rotation));
    vector_for_angle_y(angle)
}
