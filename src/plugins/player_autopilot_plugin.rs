use bevy::prelude::*;

#[derive(Default)]
pub struct PlayerAutopilotPlugin;

impl Plugin for PlayerAutopilotPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(player_autopilot_system);
    }
}

fn player_autopilot_system() {
    // TODO: listen to
}
