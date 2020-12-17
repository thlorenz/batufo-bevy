use bevy::prelude::*;
use pathfinding::prelude::*;

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
    let mut path: Option<Vec<(u32, u32)>> = None;
    if let Some(hero_tile) = &state.hero_tile {
        for event in hover_event_reader.iter(&hover_events) {
            path = path_finder.path(false, event.0.position.col_row(), hero_tile.col_row());
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
    pub fn tile_from_translation(&self, pos: &Vec3) -> Option<TilePosition> {
        let wp: WorldPosition = pos.into();
        let local_path = self.local_path.as_ref().expect(INIT_PATHFINDER_MSG);
        wp.to_tile_position(local_path.tile_size)
    }

    pub fn translation_from_tile(&self, tp: &TilePosition) -> Vec3 {
        let local_path = self.local_path.as_ref().expect(INIT_PATHFINDER_MSG);
        let wp = tp.to_world_position(local_path.tile_size);
        wp.into()
    }

    pub fn translation_from_col_row(&self, (col, row): (u32, u32)) -> Vec3 {
        let local_path = self.local_path.as_ref().expect(INIT_PATHFINDER_MSG);
        self.translation_from_tile(&TilePosition::centered(col, row, local_path.tile_size))
    }

    pub fn path(
        &self,
        allow_diagonals: bool,
        start: (u32, u32),
        end: (u32, u32),
    ) -> Option<Vec<(u32, u32)>> {
        let local_path = self.local_path.as_ref().expect(INIT_PATHFINDER_MSG);
        let result = bfs(
            &start,
            |&p| local_path.moves(allow_diagonals, p),
            |&p| p == end,
        );
        result
            .map(|tiles| tiles.into_iter().skip(1).collect())
            .filter(|tiles: &Vec<_>| !tiles.is_empty())
    }
}

struct LocalPath {
    tile_size: u32,
    valid_tiles: Vec<Vec<bool>>,
}

impl LocalPath {
    fn from_arena(arena: &Arena, tile_size: u32) -> Self {
        let mut valid_tiles = LocalPath::empty_grid(arena.ncols as usize, arena.nrows as usize);
        for tp in &arena.floor_tiles {
            let ref mut col = valid_tiles[tp.col as usize];
            col[tp.row as usize] = true;
        }

        Self {
            tile_size,
            valid_tiles,
        }
    }

    fn empty_grid(ncols: usize, nrows: usize) -> Vec<Vec<bool>> {
        let mut grid: Vec<Vec<bool>> = Vec::with_capacity(ncols);
        for _ in 0..ncols {
            grid.push(LocalPath::empty_col(nrows));
        }
        grid
    }

    fn empty_col(nrows: usize) -> Vec<bool> {
        let mut col = Vec::with_capacity(nrows);
        for _ in 0..nrows {
            col.push(false);
        }
        col
    }

    fn moves(&self, allow_diagonals: bool, tile: (u32, u32)) -> Vec<(u32, u32)> {
        let (col, row) = tile;
        let mut xs: Vec<(u32, u32)> = Vec::new();
        let has_left = col > 0;
        let has_bottom = row > 0;
        if has_left {
            if allow_diagonals {
                // top-left
                xs.push((col - 1, row + 1));
            }
            // ctr-left
            xs.push((col - 1, row));
        }
        if has_bottom {
            // btm-ctr
            xs.push((col, row - 1));
            if allow_diagonals {
                // btm-right
                xs.push((col + 1, row - 1));
            }
        }
        if has_left && has_bottom && allow_diagonals {
            // btm-left
            xs.push((col - 1, row - 1));
        }

        // top-ctr
        xs.push((col, row + 1));

        if allow_diagonals {
            // top-right
            xs.push((col + 1, row + 1));
        }
        // ctr-right
        xs.push((col + 1, row));

        xs.into_iter()
            .filter(|&(col, row)| self.valid_tiles[col as usize][row as usize])
            .collect()
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
    fn find_path_1() {
        let path_finder = init_face_off_path_finder();
        let path = path_finder.path(false, (11, 19), (11, 21));
        let expected: Vec<(u32, u32)> = vec![(11, 20), (11, 21)];
        assert_eq!(path, Some(expected));
    }

    #[test]
    fn find_path_2() {
        let path_finder = init_face_off_path_finder();
        let path = path_finder.path(false, (5, 25), (62, 25));
        assert_eq!(path.map(|x| x.len()), Some(71));
    }
}
