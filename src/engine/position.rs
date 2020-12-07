use bevy::math::{Rect, Vec2,};

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

    pub fn to_world_point(&self, tile_size: u32) -> Vec2 {
        WorldPosition::from_tile_position(self, tile_size).to_point()
    }

    #[allow(dead_code)]
    pub fn to_world_point_top_left(&self, tile_size: u32) -> Vec2 {
        self.to_world_position_top_left(tile_size).to_point()
    }

    #[allow(dead_code)]
    pub fn to_world_rect(&self, tile_size: u32) -> Rect<f32> {
        WorldPosition::from_tile_position(self, tile_size).to_rect(tile_size)
    }

    pub fn apply_velocity(&self, dt: f32, velocity: &Vec2, tile_size: u32) -> TilePosition {
        if *velocity == Vec2::new(0.0, 0.0) {
            return self.clone();
        }
        let wp = self.to_world_position(tile_size);
        let dx = velocity.x * dt;
        let dy = velocity.y * dt;
        let x = wp.x + dx;
        let y = wp.y + dy;
        let new_wp = WorldPosition::new(x, y);
        new_wp.to_tile_position(tile_size)
    }
}

#[derive(Debug, PartialEq)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
}

impl WorldPosition {
    pub fn new(x: f32, y: f32) -> WorldPosition {
        WorldPosition { x, y }
    }

    pub fn from_tile_position(tp: &TilePosition, tile_size: u32) -> WorldPosition {
        let x = (tile_size * tp.col) as f32 + tp.rel_x;
        let y = (tile_size * tp.row) as f32 + tp.rel_y;
        WorldPosition::new(x, y)
    }

    #[allow(dead_code)]
    pub fn from_tile_position_top_left(tp: &TilePosition, tile_size: u32) -> WorldPosition {
        let ht = tile_size as f32 / 2.0;
        let x = (tile_size * tp.col) as f32 + tp.rel_x - ht;
        let y = (tile_size * tp.row) as f32 + tp.rel_y - ht;
        WorldPosition::new(x, y)
    }

    #[allow(dead_code)]
    pub fn to_tile_position(&self, tile_size: u32) -> TilePosition {
        let tile_size = tile_size as f32;
        let col = (self.x / tile_size).floor() as u32;
        let row = (self.y / tile_size).floor() as u32;
        let rel_x = self.x % tile_size;
        let rel_y = self.y % tile_size;
        TilePosition::new(col, row, rel_x, rel_y)
    }

    pub fn to_point(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn to_rect(&self, tile_size: u32) -> Rect<f32> {
        let ctr: Vec2 = self.to_point();
        let ht = (tile_size / 2) as f32;
        let left = ctr.x - ht;
        let bottom = ctr.y - ht;
        Rect { left, bottom, right: left + tile_size as f32, top: bottom + tile_size as f32 }
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
                WorldPosition::new(210.0, 210.0),
                "to_world_position"
            );
            assert_eq!(
                tp.to_world_point(TILE_SIZE),
                Vec2::new(210.0, 210.0),
                "to_world_point"
            );
            let left = 200.;
            let bottom = 200.;
            assert_eq!(
                tp.to_world_rect(TILE_SIZE),
                Rect { left, bottom, right: left + TILE_SIZE as f32, top: bottom + TILE_SIZE as f32 },
                "to_world_rect"
            );
        }

        #[test]
        fn tile_world_round_trips() {
            let wp0 = WorldPosition { x: 210.0, y: 240.0 };
            let tp = wp0.to_tile_position(TILE_SIZE);
            let wp1 = tp.to_world_position(TILE_SIZE);
            assert_eq!(wp0, wp1);

            let wp0 = WorldPosition { x: 240.0, y: 241.0 };
            let tp = wp0.to_tile_position(TILE_SIZE);
            let wp1 = tp.to_world_position(TILE_SIZE);
            assert_eq!(wp0, wp1);

            let wp0 = WorldPosition { x: 10.0, y: 21.0 };
            let tp = wp0.to_tile_position(5);
            let wp1 = tp.to_world_position(5);
            assert_eq!(wp0, wp1);
        }

        mod apply_velocity_tile_size_on_y {
            use super::*;

            #[test]
            fn dt_1_0() {
                let tp0 = TilePosition::new(10, 10, 10.0, 10.0);
                let velocity = Vec2::new(0.0, TILE_SIZE as f32);

                let dt = 1.0;
                let tp1 = tp0.apply_velocity(dt, &velocity, TILE_SIZE);
                let tp2 = tp1.apply_velocity(dt, &velocity, TILE_SIZE);
                assert_eq!(
                    tp1,
                    TilePosition {
                        col: 10,
                        row: 11,
                        rel_x: 10.0,
                        rel_y: 10.0,
                    }
                );
                assert_eq!(
                    tp2,
                    TilePosition {
                        col: 10,
                        row: 12,
                        rel_x: 10.0,
                        rel_y: 10.0,
                    }
                );
            }

            #[test]
            fn dt_1_5() {
                let tp0 = TilePosition::new(10, 10, 10.0, 10.0);
                let velocity = Vec2::new(0.0, TILE_SIZE as f32);

                let dt = 1.5;
                let tp1 = tp0.apply_velocity(dt, &velocity, TILE_SIZE);
                let tp2 = tp1.apply_velocity(dt, &velocity, TILE_SIZE);

                assert_eq!(
                    tp1,
                    TilePosition {
                        col: 10,
                        row: 12,
                        rel_x: 10.0,
                        rel_y: 0.0,
                    }
                );
                assert_eq!(
                    tp2,
                    TilePosition {
                        col: 10,
                        row: 13,
                        rel_x: 10.0,
                        rel_y: 10.0,
                    }
                );
            }

            #[test]
            fn dt_16_velocity_neg_y() {
                let tp0 = TilePosition::new(10, 10, 10.0, 10.0);
                let velocity = Vec2::new(0.0, -1.0);
                let dt = 16.0;
                let tp1 = tp0.apply_velocity(dt, &velocity, TILE_SIZE);
                assert_eq!(
                    tp1,
                    TilePosition {
                        col: 10,
                        row: 9,
                        rel_x: 10.0,
                        rel_y: 14.0,
                    }
                );
            }
        }
    }
}
