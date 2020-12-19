use bevy::prelude::*;

use crate::{
    ai::find_path,
    arena::Tilepath,
    ecs::{events::HoveredTileChangedEvent, resources::TileState},
};

#[derive(Default)]
pub struct PathFinderPlugin;

impl Plugin for PathFinderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(path_to_hero_from_hovered_tile_system);
    }
}

fn path_to_hero_from_hovered_tile_system(
    mut hover_event_reader: Local<EventReader<HoveredTileChangedEvent>>,
    hover_events: Res<Events<HoveredTileChangedEvent>>,
    tilepath: Res<Tilepath>,
    mut state: ResMut<TileState>,
) {
    let mut path: Option<Vec<(u32, u32)>> = None;
    if let Some(hero_tile) = &state.hero_tile {
        for event in hover_event_reader.iter(&hover_events) {
            path = find_path(
                &tilepath.valid_tiles,
                true,
                event.0.position.col_row(),
                hero_tile.col_row(),
            );
        }
    }
    state.path_hovered_to_hero = path;
}
