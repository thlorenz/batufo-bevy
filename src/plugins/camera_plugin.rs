use bevy::{prelude::*, render::camera::Camera};

use crate::ecs::components::Hero;

use super::game_plugin::GameProps;

#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_camera)
            .add_system(camera_follow_system);
    }
}

fn setup_camera(commands: &mut Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}

fn camera_follow_system(
    time: Res<Time>,
    game_props: Res<GameProps>,
    players: Query<&Transform, With<Hero>>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    let dt = time.delta_seconds();
    for player in players.iter() {
        let lerp = (2.5 * dt).min(game_props.cameras.platform_lerp);
        for mut camera in cameras.iter_mut() {
            let dx = (player.translation.x - camera.translation.x) * lerp;
            let dy = (player.translation.y - camera.translation.y) * lerp;
            camera.translation.x += dx;
            camera.translation.y += dy;
        }
    }
}
