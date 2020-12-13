use bevy::prelude::*;
use bevy_mod_picking::{Group, PickState, PickingPlugin};

use crate::ecs::components::FloorTile;
use crate::engine::position::TilePosition;

#[derive(Default)]
struct TileInteractionState {
    hovered_tile: Option<TilePosition>,
}

#[derive(Default)]
pub struct TileInteractionPlugin;

impl Plugin for TileInteractionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(TileInteractionState::default())
            .add_plugin(PickingPlugin)
            .add_startup_system(config_pickstate_system)
            .add_system(toggle_pickstate_system)
            .add_system(highlight_tile_system);
    }
}

fn config_pickstate_system(mut pick_state: ResMut<PickState>) {
    // TODO: part of game config?
    pick_state.enabled = false;
}

fn toggle_pickstate_system(keyboard_input: Res<Input<KeyCode>>, mut pick_state: ResMut<PickState>) {
    if keyboard_input.just_pressed(KeyCode::P) {
        pick_state.enabled = !pick_state.enabled;
    }
}

fn highlight_tile_system(
    pick_state: Res<PickState>,
    mut state: ResMut<TileInteractionState>,
    mut query: Query<(Entity, &FloorTile, &mut Transform)>,
) {
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (entity, square, mut transform) in query.iter_mut() {
        if Some(entity) == top_entity {
            if state.hovered_tile.is_none() || *state.hovered_tile.as_ref().unwrap() != square.0 {
                println!("Hovering {}", square.0);
                state.hovered_tile = Some(square.0.clone());
                transform.translation.y = 0.1;
            }
        } else {
            transform.translation.y = 0.0;
        };
    }
}
