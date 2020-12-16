use bevy::prelude::*;
use bevy_mod_picking::{Group, PickState, PickingPlugin};

use crate::ecs::components::FloorTile;
use crate::ecs::events::HoveredTileChangedEvent;
use crate::ecs::resources::{EntityTile, TileState};

#[derive(Default)]
pub struct TileInteractionPlugin;

impl Plugin for TileInteractionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(TileState::default())
            .add_event::<HoveredTileChangedEvent>()
            .add_plugin(PickingPlugin)
            .add_startup_system(config_pickstate_system)
            .add_system(toggle_pickstate_system)
            .add_system(hovered_tile_changed_emitter)
            .add_system(highlight_hovered_tile_system);
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

fn hovered_tile_changed_emitter(
    pick_state: Res<PickState>,
    mut state: ResMut<TileState>,
    query: Query<(Entity, &FloorTile)>,
    mut events: ResMut<Events<HoveredTileChangedEvent>>,
) {
    match pick_state.top(Group::default()) {
        Some((top_entity, _)) => {
            for (entity, floor_tile) in query.iter() {
                if entity == *top_entity
                    && (state.hovered_tile.is_none()
                        || state.hovered_tile.as_ref().unwrap().position != floor_tile.0)
                {
                    let entity_tile: EntityTile = (entity, floor_tile.0.clone()).into();

                    state.hovered_tile = Some(entity_tile.clone());
                    events.send(HoveredTileChangedEvent(entity_tile));
                }
            }
        }
        _ => {}
    }
}

#[derive(Default)]
struct CurrentHighlight(Option<Entity>);

fn highlight_hovered_tile_system(
    mut event_reader: Local<EventReader<HoveredTileChangedEvent>>,
    events: Res<Events<HoveredTileChangedEvent>>,
    mut query: Query<&mut Transform, With<FloorTile>>,
    mut current_highlight: Local<CurrentHighlight>,
) {
    for event in event_reader.iter(&events) {
        if let Some(entity) = current_highlight.0 {
            if entity == event.0.entity {
                continue;
            }
            if let Ok(mut transform) = query.get_mut(entity) {
                transform.translation.y = 0.0;
            }
        }

        if let Ok(mut transform) = query.get_mut(event.0.entity) {
            transform.translation.y = 0.1;
            current_highlight.0 = Some(event.0.entity);
        }
    }
}
