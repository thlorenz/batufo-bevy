use crate::arena::levels::Levels;
use crate::arena::tilemap::{needs_floor_tile, Tile, Tilemap};
use crate::engine::TilePosition;
use std::error::Error;
use std::fmt;

#[derive(fmt::Debug)]
pub struct Arena {
    pub floor_tiles: Vec<TilePosition>,
    pub walls: Vec<TilePosition>,
    pub player: TilePosition,
    pub ncols: u32,
    pub nrows: u32,
}

impl Arena {
    pub fn new(
        floor_tiles: Vec<TilePosition>,
        walls: Vec<TilePosition>,
        player: TilePosition,
        ncols: u32,
        nrows: u32,
    ) -> Arena {
        Arena {
            floor_tiles,
            walls,
            player,
            ncols,
            nrows,
        }
    }

    pub fn from_tilemap(tilemap: Tilemap) -> Arena {
        let nrows = tilemap.nrows;
        let ncols = tilemap.ncols;
        let mut floor_tiles: Vec<TilePosition> = Vec::new();
        let mut walls: Vec<TilePosition> = Vec::new();
        let mut player: Option<TilePosition> = None;
        for row in 0..nrows {
            for col in 0..ncols {
                let idx: usize = (row * ncols + col) as usize;
                let tile = tilemap.tiles.get(idx).expect("should have tile at idx");
                if needs_floor_tile(tile) {
                    floor_tiles.push(TilePosition::centered(col, row, tilemap.tile_size))
                }
                match tile {
                    Tile::OutOfBounds => {}
                    Tile::Empty => {}
                    Tile::Hole => {}
                    Tile::Wall => walls.push(TilePosition::centered(col, row, tilemap.tile_size)),
                    Tile::Player => {
                        player = Some(TilePosition::centered(col, row, tilemap.tile_size))
                    }
                    Tile::Medkit => {}
                    Tile::Shield => {}
                    Tile::Bomb => {}
                    Tile::Teleport1 => {}
                    Tile::Teleport2 => {}
                    Tile::Teleport3 => {}
                    Tile::Teleport4 => {}
                    Tile::Teleport5 => {}
                    Tile::Teleport6 => {}
                    Tile::Teleport7 => {}
                    Tile::Teleport8 => {}
                }
            }
        }
        let player = player.expect("Terrain is missing player");
        Arena::new(floor_tiles, walls, player, ncols, nrows)
    }

    pub fn for_level(level_name: &'static str, tile_size: u32) -> Result<Arena, Box<dyn Error>> {
        let levels = Levels::new();
        let face_off = levels
            .get_level(level_name)
            .ok_or(format!("level not found '{}'", level_name))?;
        let tilemap = Tilemap::new(face_off.terrain, tile_size)?;
        Ok(Arena::from_tilemap(tilemap))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TILE_SIZE: u32 = 2;
    const CENTER: f32 = TILE_SIZE as f32 / 2.0;
    #[test]
    fn floor_tiles() {
        let small_terrain = "
====
=p =
====
";

        let tilemap =
            Tilemap::new(small_terrain, TILE_SIZE).expect("should return correct tilemap");
        let arena = Arena::from_tilemap(tilemap);
        let floor_tiles = &arena.floor_tiles;
        assert_eq!(floor_tiles.len(), 2, "has two floor tiles");

        let tile0 = floor_tiles.get(0).unwrap();
        let tile1 = floor_tiles.get(1).unwrap();
        assert_eq!(tile0, &TilePosition::new(1, 1, CENTER, CENTER), "tile0");
        assert_eq!(tile1, &TilePosition::new(2, 1, CENTER, CENTER), "tile1");

        print!("{:?}", arena)
    }
}
