use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use crate::ecs::components::{Hero, Velocity};

#[derive(Default)]
pub struct PlayerInput;

impl Plugin for PlayerInput {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(keyboard_input_system);
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Hero>>,
) {
    let dv = 0.2;
    for mut velocity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            velocity.0.y += dv;
        }
        if keyboard_input.pressed(KeyCode::S) {
            velocity.0.y -= dv;
        }
        if keyboard_input.pressed(KeyCode::A) {
            velocity.0.x -= dv;
        }
        if keyboard_input.pressed(KeyCode::D) {
            velocity.0.x += dv;
        }
        if velocity.0.y < 0.0 {
            velocity.0.y = 0.0;
        }
    }
}
