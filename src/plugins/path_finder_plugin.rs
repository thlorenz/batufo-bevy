use bevy::prelude::*;
use bevy::utils::HashSet;
use pathfinding::prelude::bfs;

use crate::arena::arena::Arena;
use crate::ecs::events::HoveredTileChangedEvent;
use crate::ecs::resources::TileState;
use crate::engine::position::TilePosition;
use crate::plugins::game_plugin::GameRender;

#[derive(Default)]
pub struct PathFinderPlugin;

impl Plugin for PathFinderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PathFinder>()
            .add_startup_system(path_finder_setup)
            .add_system(path_to_hero_from_hovered_tile_system);
    }
}

fn path_finder_setup(arena: Res<Arena>, mut path_finder: ResMut<PathFinder>) {
    path_finder.local_path = Some(LocalPath::from_arena(&arena));
}

fn path_to_hero_from_hovered_tile_system(
    mut hover_event_reader: Local<EventReader<HoveredTileChangedEvent>>,
    hover_events: Res<Events<HoveredTileChangedEvent>>,
    path_finder: Res<PathFinder>,
    game_render: Res<GameRender>,
    mut state: ResMut<TileState>,
) {
    let mut path: Option<Vec<TilePosition>> = None;
    if let Some(hero_tile) = &state.hero_tile {
        for event in hover_event_reader.iter(&hover_events) {
            path = path_finder.path(game_render.tile_size, false, &event.0.position, &hero_tile);
        }
    }
    state.path_hovered_to_hero = path;
}

#[derive(Default)]
struct PathFinder {
    local_path: Option<LocalPath>,
}

impl PathFinder {
    fn path(
        &self,
        tile_size: u32,
        allow_diagonals: bool,
        start: &TilePosition,
        end: &TilePosition,
    ) -> Option<Vec<TilePosition>> {
        match &self.local_path {
            Some(local_path) => {
                let start_idx = start.tile_idx(local_path.nrows);
                let end_idx = end.tile_idx(local_path.nrows);
                let result = bfs(
                    &start_idx,
                    |p| local_path.moves(allow_diagonals, *p),
                    |p| *p == end_idx,
                );
                if let Some(result) = result {
                    Some(
                        result
                            .iter()
                            .map(|idx| {
                                TilePosition::from_tile_idx_centered(
                                    local_path.nrows,
                                    tile_size,
                                    *idx,
                                )
                            })
                            .collect(),
                    )
                } else {
                    None
                }
            }
            None => panic!("Need to init PathFinder before looking for paths"),
        }
    }
}

struct LocalPath {
    nrows: u32,
    valid_tiles: HashSet<u32>,
}

impl LocalPath {
    fn from_arena(arena: &Arena) -> Self {
        let nrows = arena.nrows;
        let wall_idxs: HashSet<u32> = arena.walls.iter().map(|tp| tp.tile_idx(nrows)).collect();
        let valid_tiles: HashSet<u32> = arena
            .floor_tiles
            .iter()
            .map(|tp| tp.tile_idx(nrows))
            .filter(|idx| !wall_idxs.contains(idx))
            .collect();
        Self { nrows, valid_tiles }
    }

    fn moves(&self, allow_diagonals: bool, tile_idx: u32) -> Vec<u32> {
        let mut xs: Vec<TilePosition> = Vec::new();
        let TilePosition { col, row, .. } = TilePosition::from_tile_idx(self.nrows, tile_idx);
        let has_left = col > 0;
        let has_bottom = row > 0;
        if has_left {
            if allow_diagonals {
                // top-left
                xs.push(TilePosition::origin(col - 1, row + 1));
            }
            // ctr-left
            xs.push(TilePosition::origin(col - 1, row));
        }
        if has_bottom {
            // btm-ctr
            xs.push(TilePosition::origin(col, row - 1));
            if allow_diagonals {
                // btm-right
                xs.push(TilePosition::origin(col + 1, row - 1));
            }
        }
        if has_left && has_bottom && allow_diagonals {
            // btm-left
            xs.push(TilePosition::origin(col - 1, row - 1));
        }

        // top-ctr
        xs.push(TilePosition::origin(col, row + 1));

        if allow_diagonals {
            // top-right
            xs.push(TilePosition::origin(col + 1, row + 1));
        }
        // ctr-right
        xs.push(TilePosition::origin(col + 1, row));

        xs.iter()
            .map(|tp| tp.tile_idx(self.nrows))
            .filter(|idx| self.valid_tiles.contains(idx))
            .collect()
    }
}
