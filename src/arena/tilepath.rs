use super::Arena;

pub struct Tilepath {
    pub valid_tiles: Vec<Vec<bool>>,
}

impl Tilepath {
    pub fn from_arena(arena: &Arena) -> Self {
        let mut valid_tiles = Tilepath::empty_grid(arena.ncols as usize, arena.nrows as usize);
        for tp in &arena.floor_tiles {
            let ref mut col = valid_tiles[tp.col as usize];
            col[tp.row as usize] = true;
        }

        Self { valid_tiles }
    }

    fn empty_grid(ncols: usize, nrows: usize) -> Vec<Vec<bool>> {
        let mut grid: Vec<Vec<bool>> = Vec::with_capacity(ncols);
        grid.resize(ncols, Tilepath::empty_col(nrows));
        grid
    }

    fn empty_col(nrows: usize) -> Vec<bool> {
        let mut col = Vec::with_capacity(nrows);
        col.resize(nrows, false);
        col
    }
}
