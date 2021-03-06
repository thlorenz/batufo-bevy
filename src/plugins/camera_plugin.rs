use crate::engine::physics::vector_for_rotation_y;
use bevy::{
    prelude::*,
    render::camera::{Camera, PerspectiveProjection},
};

use crate::ecs::components::Hero;

use super::game_plugin::GameCameras;
use bevy::input::mouse::MouseWheel;

struct CameraProperties {
    distance_to_hero: f32,
}
impl Default for CameraProperties {
    fn default() -> Self {
        Self {
            distance_to_hero: 5.0,
        }
    }
}

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_camera.system())
            .add_system(camera_follow_system.system())
            .add_system(camera_zoom_system.system());
    }
}

fn setup_camera(commands: &mut Commands) {
    commands
        .spawn(Camera3dBundle {
            perspective_projection: PerspectiveProjection {
                near: 1.0,
                far: 5000.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(CameraProperties::default());
}

fn camera_follow_system(
    time: Res<Time>,
    game_cameras: Res<GameCameras>,
    players: Query<&Transform, With<Hero>>,
    mut cameras: Query<(&mut Transform, &CameraProperties), With<Camera>>,
) {
    let dt = time.delta_seconds();
    let lerp = (10.0 * dt).min(game_cameras.platform_lerp);
    for player in players.iter() {
        let p_translation = player.translation;
        let Vec3 { x, z, .. } = vector_for_rotation_y(&player.rotation);

        for (mut camera, props) in cameras.iter_mut() {
            let camera_target = Vec3::new(
                p_translation.x + (props.distance_to_hero * x),
                p_translation.y + props.distance_to_hero * 0.3,
                p_translation.z + (props.distance_to_hero * z),
            );
            let dx = (camera_target.x - camera.translation.x) * lerp;
            let dy = (camera_target.y - camera.translation.y) * lerp;
            let dz = (camera_target.z - camera.translation.z) * lerp;
            camera.translation.x += dx;
            camera.translation.y += dy;
            camera.translation.z += dz;
            camera.look_at(p_translation, Vec3::unit_y());
        }
    }
}
#[derive(Default)]
struct MouseState {
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

fn camera_zoom_system(
    mut state: Local<MouseState>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut cameras: Query<&mut CameraProperties, With<Camera>>,
) {
    let scroll_factor = 0.02;
    let min_distance = 3.0;
    let max_distance = 40.0;

    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        for mut props in cameras.iter_mut() {
            props.distance_to_hero -= event.y * scroll_factor;
            if props.distance_to_hero > max_distance {
                props.distance_to_hero = max_distance;
            }
            if props.distance_to_hero < min_distance {
                props.distance_to_hero = min_distance;
            }
        }
    }
}
