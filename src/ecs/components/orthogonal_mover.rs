use crate::animations::MovementAnimation;

/// Possible state changes:
///   Idle -> Moving
///   Moving -> Idle
///   Note that if a Frozen Component is added this will override any movement state of a movement
///   component.
pub enum MovementState {
    Idle,
    Moving(MovementAnimation),
}

pub struct OrthogonalMovement {
    pub step_factor: f32,
    pub center_y: f32,
    pub state: MovementState,
}

impl Default for OrthogonalMovement {
    fn default() -> Self {
        OrthogonalMovement {
            step_factor: 1.5,
            center_y: 0.5,
            state: MovementState::Idle,
        }
    }
}
