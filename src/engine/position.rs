use bevy::{
    math::{Rect, Vec3},
    prelude::Transform,
};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct TilePosition {
    pub col: u32,
    pub row: u32,
    pub rel_x: f32,
    pub rel_y: f32,
}

impl TilePosition {
    pub fn new(col: u32, row: u32, rel_x: f32, rel_y: f32) -> TilePosition {
        TilePosition {
            col,
            row,
            rel_x,
            rel_y,
        }
    }

    pub fn centered(col: u32, row: u32, tile_size: u32) -> TilePosition {
        let rel_x = tile_size as f32 / 2.0;
        let rel_y = tile_size as f32 / 2.0;
        TilePosition::new(col, row, rel_x, rel_y)
    }

    #[allow(dead_code)]
    pub fn to_world_position_top_left(&self, tile_size: u32) -> WorldPosition {
        WorldPosition::from_tile_position_top_left(self, tile_size)
    }

    pub fn to_world_position(&self, tile_size: u32) -> WorldPosition {
        WorldPosition::from_tile_position(self, tile_size)
    }

    #[allow(dead_code)]
    pub fn to_world_rect(&self, tile_size: u32) -> Rect<f32> {
        WorldPosition::from_tile_position(self, tile_size).to_rect(tile_size)
    }
}

impl Display for TilePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}+{}, {}+{})",
            self.col, self.rel_x, self.row, self.rel_y
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WorldPosition {
    pub fn new(x: f32, y: f32, z: f32) -> WorldPosition {
        WorldPosition { x, y, z }
    }

    // When converting from tile position we transpose 2D positions to a 3D
    pub fn from_tile_position(tp: &TilePosition, tile_size: u32) -> WorldPosition {
        let x = (tile_size * tp.col) as f32 + tp.rel_x;
        let z = (tile_size * tp.row) as f32 + tp.rel_y;
        WorldPosition::new(x, 0.0, -z)
    }

    #[allow(dead_code)]
    pub fn from_tile_position_top_left(tp: &TilePosition, tile_size: u32) -> WorldPosition {
        let ht = tile_size as f32 / 2.0;
        let x = (tile_size * tp.col) as f32 + tp.rel_x - ht;
        let z = (tile_size * tp.row) as f32 + tp.rel_y - ht;
        WorldPosition::new(x, 0.0, -z)
    }

    // When converting to tile position we transpose 3D positions to a 2D
    #[allow(dead_code)]
    pub fn to_tile_position(&self, tile_size: u32) -> TilePosition {
        let tile_size = tile_size as f32;
        let col = (self.x / tile_size).floor() as u32;
        let row = ((-self.z) / tile_size).floor() as u32;
        let rel_x = self.x % tile_size;
        let rel_y = (-self.z) % tile_size;
        TilePosition::new(col, row, rel_x, rel_y)
    }

    pub fn to_rect(&self, tile_size: u32) -> Rect<f32> {
        let ht = (tile_size / 2) as f32;
        let left = self.x - ht;
        let bottom = (-self.z) - ht;
        Rect {
            left,
            bottom,
            right: left + tile_size as f32,
            top: bottom + tile_size as f32,
        }
    }
}

//
// Bevy specific
//
impl Into<Vec3> for WorldPosition {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Into<Transform> for WorldPosition {
    fn into(self) -> Transform {
        Transform::from_translation(self.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TILE_SIZE: u32 = 20;

    mod tile_position {
        use super::*;

        #[test]
        fn init() {
            let tp = TilePosition::new(1, 1, 10.0, 10.0);
            let centered = TilePosition::centered(1, 1, TILE_SIZE);
            assert_eq!(tp, centered, "new(1, 1, 10.0, 10.0) == centered(1, 1, 20)")
        }

        #[test]
        fn conversions() {
            let tp = TilePosition::new(10, 10, 10.0, 10.0);
            assert_eq!(
                tp.to_world_position(TILE_SIZE),
                WorldPosition::new(210.0, 0.0, 210.0),
                "to_world_position"
            );
            let left = 200.;
            let bottom = 200.;
            assert_eq!(
                tp.to_world_rect(TILE_SIZE),
                Rect {
                    left,
                    bottom,
                    right: left + TILE_SIZE as f32,
                    top: bottom + TILE_SIZE as f32,
                },
                "to_world_rect"
            );
        }

        #[test]
        fn tile_world_round_trips() {
            let wp0 = WorldPosition {
                x: 210.0,
                y: 0.0,
                z: 240.0,
            };
            let tp = wp0.to_tile_position(TILE_SIZE);
            let wp1 = tp.to_world_position(TILE_SIZE);
            assert_eq!(wp0, wp1);

            let wp0 = WorldPosition {
                x: 240.0,
                y: 0.0,
                z: 241.0,
            };
            let tp = wp0.to_tile_position(TILE_SIZE);
            let wp1 = tp.to_world_position(TILE_SIZE);
            assert_eq!(wp0, wp1);

            let wp0 = WorldPosition {
                x: 10.0,
                y: 0.0,
                z: 21.0,
            };
            let tp = wp0.to_tile_position(5);
            let wp1 = tp.to_world_position(5);
            assert_eq!(wp0, wp1);
        }
    }
}
