use bevy::{
    prelude::*,
    render::camera::{Camera, PerspectiveProjection},
};

use crate::ecs::components::Hero;

use super::game_plugin::GameCameras;

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_camera)
            .add_system(camera_follow_system);
    }
}

fn setup_camera(commands: &mut Commands) {
    commands.spawn(Camera3dBundle {
        perspective_projection: PerspectiveProjection {
            near: 1.0,
            far: 5000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn camera_follow_system(
    time: Res<Time>,
    game_cameras: Res<GameCameras>,
    players: Query<&Transform, With<Hero>>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    let dt = time.delta_seconds();
    let lerp = (1.5 * dt).min(game_cameras.platform_lerp);
    for player in players.iter() {
        let p = player.translation;
        for mut camera in cameras.iter_mut() {
            let camera_target = Vec3::new(p.x, p.y + 180.0, p.z + 400.0);
            let dx = (camera_target.x - camera.translation.x) * lerp;
            let dy = (camera_target.y - camera.translation.y) * lerp;
            let dz = (camera_target.z - camera.translation.z) * lerp;
            camera.translation.x += dx;
            camera.translation.y += dy;
            camera.translation.z += dz;
            camera.look_at(p, Vec3::unit_y());
        }
    }
}
