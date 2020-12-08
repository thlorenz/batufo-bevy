use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    render::camera::PerspectiveProjection,
};

use crate::ecs::components::{HeadLights, Hero, Velocity};

#[derive(Default)]
pub struct PlayerInput;

impl Plugin for PlayerInput {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(velocity_input_system)
            .add_system(light_input_system);
    }
}

fn velocity_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Hero>>,
) {
    let dv = 0.2;
    for mut velocity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            velocity.0.z -= dv;
        }
        if keyboard_input.pressed(KeyCode::S) {
            velocity.0.z += dv;
        }
        if keyboard_input.pressed(KeyCode::A) {
            velocity.0.x -= dv;
        }
        if keyboard_input.pressed(KeyCode::D) {
            velocity.0.x += dv;
        }
        /*
        if velocity.0.z > 0.0 {
            velocity.0.z = 0.0;
        }
        */
    }
}

/// The below doesn't work at this point as we are unable to get to the LightBundle
/// which is a child of our player.
fn light_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut lights_query: Query<(&mut Light, &mut HeadLights)>,
    mut perspective_query: Query<&mut PerspectiveProjection>,
) {
    let mut head_lights_on: bool = false;
    if keyboard_input.just_pressed(KeyCode::L) {
        for (mut light, mut head_lights) in lights_query.iter_mut() {
            head_lights.0 = !head_lights.0;
            head_lights_on = head_lights.0;

            if head_lights_on {
                light.color = Color::WHITE;
            } else {
                light.color = Color::BLACK;
            }
        }

        // TODO: Currently has no effect, may need to update camera orthographic projection
        // directly
        for mut persp in perspective_query.iter_mut() {
            if head_lights_on {
                persp.far = 5000.0;
            } else {
                persp.far = 500.0;
            }
        }
    }
}
