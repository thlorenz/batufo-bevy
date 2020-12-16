use bevy::prelude::*;
use bevy::utils::HashSet;
use pathfinding::prelude::bfs;

use crate::arena::arena::Arena;
use crate::ecs::events::HoveredTileChangedEvent;
use crate::ecs::resources::TileState;
use crate::engine::position::{TilePosition, WorldPosition};
use crate::plugins::game_plugin::GameRender;

const INIT_PATHFINDER_MSG: &str = "Need to init PathFinder before looking for paths";

#[derive(Default)]
pub struct PathFinderPlugin;

impl Plugin for PathFinderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PathFinder>()
            .add_startup_system(path_finder_setup)
            .add_system(path_to_hero_from_hovered_tile_system);
    }
}

fn path_finder_setup(
    arena: Res<Arena>,
    game_render: Res<GameRender>,
    mut path_finder: ResMut<PathFinder>,
) {
    path_finder.local_path = Some(LocalPath::from_arena(&arena, game_render.tile_size));
}

fn path_to_hero_from_hovered_tile_system(
    mut hover_event_reader: Local<EventReader<HoveredTileChangedEvent>>,
    hover_events: Res<Events<HoveredTileChangedEvent>>,
    path_finder: Res<PathFinder>,
    mut state: ResMut<TileState>,
) {
    let mut path: Option<Vec<TilePosition>> = None;
    if let Some(hero_tile) = &state.hero_tile {
        for event in hover_event_reader.iter(&hover_events) {
            path = path_finder.path(false, &event.0.position, &hero_tile);
        }
    }
    state.path_hovered_to_hero = path;
}

#[derive(Default)]
pub struct PathFinder {
    local_path: Option<LocalPath>,
}

impl PathFinder {
    // TODO: this kind of conversion function should live in its own resource
    pub fn tile_from_translation(&self, pos: &Vec3) -> TilePosition {
        let wp: WorldPosition = pos.into();
        let local_path = self.local_path.as_ref().expect(INIT_PATHFINDER_MSG);
        wp.to_tile_position(local_path.tile_size)
    }

    pub fn translation_from_tile(&self, tp: &TilePosition) -> Vec3 {
        let local_path = self.local_path.as_ref().expect(INIT_PATHFINDER_MSG);
        let wp = tp.to_world_position(local_path.tile_size);
        wp.into()
    }

    pub fn path(
        &self,
        allow_diagonals: bool,
        start: &TilePosition,
        end: &TilePosition,
    ) -> Option<Vec<TilePosition>> {
        let local_path = self.local_path.as_ref().expect(INIT_PATHFINDER_MSG);
        let start_idx = start.tile_idx(local_path.nrows);
        let end_idx = end.tile_idx(local_path.nrows);
        let result = bfs(
            &start_idx,
            |p| local_path.moves(allow_diagonals, *p),
            |p| *p == end_idx,
        );
        if let Some(result) = result {
            let tiles: Vec<TilePosition> = result
                .iter()
                .skip(1)
                .map(|idx| {
                    TilePosition::from_tile_idx_centered(
                        local_path.nrows,
                        local_path.tile_size,
                        *idx,
                    )
                })
                .collect();
            if tiles.is_empty() {
                None
            } else {
                Some(tiles)
            }
        } else {
            None
        }
    }
}

struct LocalPath {
    nrows: u32,
    tile_size: u32,
    valid_tiles: HashSet<u32>,
}

impl LocalPath {
    fn from_arena(arena: &Arena, tile_size: u32) -> Self {
        let nrows = arena.nrows;
        let valid_tiles: HashSet<u32> = arena
            .floor_tiles
            .iter()
            .map(|tp| tp.tile_idx(nrows))
            .collect();
        Self {
            nrows,
            tile_size,
            valid_tiles,
        }
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

        let move_idxs = xs
            .iter()
            .map(|tp| tp.tile_idx(self.nrows))
            .filter(|idx| self.valid_tiles.contains(idx))
            .collect();

        move_idxs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn init_face_off_path_finder() -> PathFinder {
        let tile_size = 1;
        let arena = Arena::for_level("face off", tile_size).expect("FATAL: unable to create arena");

        let local_path = Some(LocalPath::from_arena(&arena, tile_size));
        PathFinder { local_path }
    }

    #[test]
    fn player_to_the_left() {
        let path_finder = init_face_off_path_finder();
        let path = path_finder.path(false, &(11, 19).into(), &(11, 21).into());
        let expected: Vec<TilePosition> = vec![
            (11.0 + 0.5, 20.0 + 0.5).into(),
            (11.0 + 0.5, 21.0 + 0.5).into(),
        ];
        assert_eq!(path, Some(expected));
    }
}
