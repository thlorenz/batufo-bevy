use bevy::{pbr::AmbientLight, prelude::*};

#[derive(Default)]
pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ambient_light_system.system());
    }
}

fn setup_ambient_light_system(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.color = Color::rgba(0.8, 0.0, 0.2, 1.0);
}
