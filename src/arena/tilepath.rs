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

    #[allow(dead_code)]
    pub fn all_valid(ncols: usize, nrows: usize) -> Self {
        let mut valid_tiles = Tilepath::empty_grid(ncols, nrows);
        for col in 0..ncols {
            for row in 0..nrows {
                valid_tiles[col][row] = true;
            }
        }
        Self { valid_tiles }
    }

    #[allow(dead_code)]
    pub fn with_invalids(ncols: usize, nrows: usize, invalid_coords: Vec<(usize, usize)>) -> Self {
        let mut tile_path = Self::all_valid(ncols, nrows);
        for (col, row) in invalid_coords {
            tile_path.valid_tiles[col][row] = false
        }
        tile_path
    }

    pub fn is_valid(&self, col: u32, row: u32) -> bool {
        self.valid_tiles[col as usize][row as usize]
    }

    pub fn empty_grid(ncols: usize, nrows: usize) -> Vec<Vec<bool>> {
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
