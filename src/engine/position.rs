use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use bevy::{
    math::{Rect, Vec3},
    prelude::Transform,
};

#[derive(Clone, PartialEq)]
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

    pub fn col_row(&self) -> (u32, u32) {
        (self.col, self.row)
    }

    pub fn centered(col: u32, row: u32, tile_size: u32) -> TilePosition {
        let rel_x = tile_size as f32 / 2.0;
        let rel_y = tile_size as f32 / 2.0;
        TilePosition::new(col, row, rel_x, rel_y)
    }

    pub fn origin(col: u32, row: u32) -> TilePosition {
        TilePosition::new(col, row, 0.0, 0.0)
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

    pub fn tile_idx(&self, ncols: u32) -> u32 {
        self.row * ncols + self.col
    }

    pub fn from_tile_idx(ncols: u32, idx: u32) -> Self {
        let row = idx / ncols;
        let col = if row == 0 { idx } else { idx % row };
        TilePosition::origin(col, row)
    }

    pub fn from_tile_idx_centered(ncols: u32, tile_size: u32, idx: u32) -> Self {
        let TilePosition { col, row, .. } = TilePosition::from_tile_idx(ncols, idx);
        TilePosition::centered(col, row, tile_size)
    }

    pub fn axes(&self) -> (f32, f32) {
        (self.col as f32 + self.rel_x, self.row as f32 + self.rel_y)
    }

    pub fn delta_to(&self, target: &TilePosition) -> (f32, f32) {
        let (x1, y1) = self.axes();
        let (x2, y2) = target.axes();
        (x2 - x1, y2 - y1)
    }

    pub fn is_same_tile(&self, other: &TilePosition) -> bool {
        self.col == other.col && self.row == other.row
    }
}

impl Debug for TilePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}+{}, {}+{})",
            self.col, self.rel_x, self.row, self.rel_y
        )
    }
}

impl Display for TilePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.col, self.row)
    }
}

impl From<(i32, i32)> for TilePosition {
    fn from((col, row): (i32, i32)) -> Self {
        assert!(col >= 0);
        assert!(row >= 0);
        TilePosition::origin(col as u32, row as u32)
    }
}

impl From<(f32, f32)> for TilePosition {
    fn from((colf, rowf): (f32, f32)) -> Self {
        assert!(colf >= 0.0);
        assert!(rowf >= 0.0);

        let col = colf as u32;
        let row = rowf as u32;
        let rel_x = colf - col as f32;
        let rel_y = rowf - row as f32;
        TilePosition::new(col as u32, row as u32, rel_x, rel_y)
    }
}

impl Into<(u32, u32)> for &TilePosition {
    fn into(self: Self) -> (u32, u32) {
        let TilePosition { col, row, .. } = self;
        (*col, *row)
    }
}

impl Into<((u32, f32), (u32, f32))> for &TilePosition {
    fn into(self: Self) -> ((u32, f32), (u32, f32)) {
        let TilePosition {
            col,
            row,
            rel_x,
            rel_y,
        } = self;
        ((*col, *rel_x), (*row, *rel_y))
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
    pub fn to_tile_position(&self, tile_size: u32) -> Option<TilePosition> {
        if self.x < 0.0 || self.z > 0.0 {
            None
        } else {
            let z = -self.z;

            let tile_size = tile_size as f32;
            let col = (self.x / tile_size).floor() as u32;
            let row = (z / tile_size).floor() as u32;
            let rel_x = self.x % tile_size;
            let rel_y = z % tile_size;
            Some(TilePosition::new(col, row, rel_x, rel_y))
        }
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

impl Into<Vec3> for &WorldPosition {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Into<Transform> for WorldPosition {
    fn into(self) -> Transform {
        Transform::from_translation(self.into())
    }
}

impl Into<Transform> for &WorldPosition {
    fn into(self) -> Transform {
        Transform::from_translation(self.into())
    }
}

impl From<&Vec3> for WorldPosition {
    fn from(vec: &Vec3) -> Self {
        WorldPosition::new(vec.x, vec.y, vec.z)
    }
}

impl From<&Transform> for WorldPosition {
    fn from(transform: &Transform) -> WorldPosition {
        let Vec3 { x, y, z } = transform.translation;
        WorldPosition::new(x, y, z)
    }
}

impl From<Transform> for WorldPosition {
    fn from(transform: Transform) -> WorldPosition {
        WorldPosition::from(&transform)
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
                WorldPosition::new(210.0, 0.0, -210.0),
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
                z: -240.0,
            };
            let tp = wp0.to_tile_position(TILE_SIZE);
            assert!(tp.is_some());
            let wp1 = tp.unwrap().to_world_position(TILE_SIZE);
            assert_eq!(wp0, wp1);

            let wp0 = WorldPosition {
                x: 240.0,
                y: 0.0,
                z: -241.0,
            };
            let tp = wp0.to_tile_position(TILE_SIZE);
            assert!(tp.is_some());
            let wp1 = tp.unwrap().to_world_position(TILE_SIZE);
            assert_eq!(wp0, wp1);

            let wp0 = WorldPosition {
                x: 10.0,
                y: 0.0,
                z: -21.0,
            };
            let tp = wp0.to_tile_position(5);
            assert!(tp.is_some());
            let wp1 = tp.unwrap().to_world_position(5);
            assert_eq!(wp0, wp1);
        }
    }
}
