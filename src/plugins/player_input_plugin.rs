use bevy::{
    input::{keyboard::KeyCode, mouse::MouseMotion, Input},
    prelude::*,
    render::camera::PerspectiveProjection,
};

use crate::ecs::components::{HeadLights, Hero, Velocity};
use crate::engine::physics::vector_for_rotation_y;

#[derive(Default)]
pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(velocity_input_system)
            .add_system(light_input_system)
            .add_system(yaw_input_system);
    }
}

fn velocity_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Transform), With<Hero>>,
) {
    let dv = 0.004;
    for (mut velocity, transform) in query.iter_mut() {
        let Vec3 { x, z, .. } = vector_for_rotation_y(transform.rotation);
        if keyboard_input.pressed(KeyCode::W) {
            velocity.0.z -= dv * z;
            velocity.0.x -= dv * x;
        }
        if keyboard_input.pressed(KeyCode::S) {
            velocity.0.z += dv * z;
            velocity.0.x += dv * x;
        }
        if keyboard_input.pressed(KeyCode::A) {
            velocity.0.x -= dv;
        }
        if keyboard_input.pressed(KeyCode::D) {
            velocity.0.x += dv;
        }
        /*
        velocity.0.x = velocity.0.x.clamp(-max_x_v, max_x_v);
        */
    }
}

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

#[derive(Default)]
struct MouseState {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

fn yaw_input_system(
    mut state: Local<MouseState>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut transform_query: Query<&mut Transform, With<Hero>>,
) {
    let rot_factor = 0.01;
    if mouse_button_input.pressed(MouseButton::Left) {
        for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
            let dx = event.delta.x;
            for mut transform in transform_query.iter_mut() {
                transform.rotate(Quat::from_rotation_y(-dx * rot_factor));
                /*
                println!(
                    "radians: {:2} degress: {:2}",
                    transform.rotation.y,
                    transform.rotation.to_axis_angle().1,
                );
                */
            }
        }
    }
}
