use bevy::prelude::*;

use super::{Direction, MovementAxis, RotationAxis};

#[allow(dead_code)]
pub struct Spin {
    axis: RotationAxis,
    start_angle: f32,
    target_angle: f32,
}

impl Spin {
    pub fn from_delta_angle(current_rotation: &Quat, axis: RotationAxis, delta_angle: f32) -> Self {
        let (start_angle, target_angle) = match axis {
            RotationAxis::X(Direction::Positive) => {
                (current_rotation.x, current_rotation.x + delta_angle)
            }
            RotationAxis::X(Direction::Negative) => {
                (current_rotation.x, current_rotation.x - delta_angle)
            }
            RotationAxis::Z(Direction::Positive) => {
                (current_rotation.z, current_rotation.z + delta_angle)
            }
            RotationAxis::Z(Direction::Negative) => {
                (current_rotation.z, current_rotation.z - delta_angle)
            }
        };
        Self {
            axis,
            start_angle,
            target_angle,
        }
    }

    fn apply(&self, total_percent: f32) -> Quat {
        let da = (self.target_angle - self.start_angle) * total_percent;
        match self.axis {
            RotationAxis::X(_) => Quat::from_rotation_x(da),
            RotationAxis::Z(_) => Quat::from_rotation_z(da),
        }
    }
}

pub struct Movement {
    axis: MovementAxis,
    start_pos: Vec3,
    end_pos: Vec3,
    max_dy: f32,
}

impl Movement {
    pub fn from_start_end(start_pos: Vec3, end_pos: Vec3, axis: MovementAxis) -> Self {
        Self {
            axis,
            start_pos,
            end_pos,
            max_dy: 0.03,
        }
    }

    fn apply(&self, Vec3 { x, y, z }: &Vec3, completed_percent: f32) -> Vec3 {
        let y_factor = completed_percent - 0.5;
        let y = y + self.max_dy * -y_factor;
        match &self.axis {
            MovementAxis::X(_) => {
                let x = (self.end_pos.x - self.start_pos.x) * completed_percent;
                Vec3::new(self.start_pos.x + x, y, *z)
            }
            MovementAxis::Z(_) => {
                let z = (self.end_pos.z - self.start_pos.z) * completed_percent;
                Vec3::new(*x, y, self.start_pos.z + z)
            }
        }
    }
}

pub struct RollingBoxAnimation {
    pub movement: Movement,
    pub spin: Spin,
    pub percent_complete: f32,
}

impl RollingBoxAnimation {
    pub fn step_percent(&mut self, transform: &mut Transform, percent: f32) -> bool {
        self.percent_complete += percent;
        if self.percent_complete > 1.0 {
            true
        } else {
            transform.translation = self
                .movement
                .apply(&transform.translation, self.percent_complete);
            transform.rotation = self.spin.apply(self.percent_complete);
            false
        }
    }
}
