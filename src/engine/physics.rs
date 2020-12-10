use bevy::math::{Quat, Vec3};

pub fn vector_for_rotation_y(rotation: Quat) -> Vec3 {
    let axis_angle = rotation.to_axis_angle();
    let angle = axis_angle.1 * axis_angle.0.y;
    let x = angle.sin();
    let z = angle.cos();
    return Vec3::new(x, 1.0, z);
}
