use std::fmt::Display;

use crisscross::{AngleRad, Grid, TileRaycaster};

use crate::{arena::Tilepath, engine::TilePosition};

pub fn create_tile_caster(ncols: u32, nrows: u32, tile_size: f32) -> TileRaycaster {
    let grid = Grid::new(ncols, nrows, tile_size as f32);
    TileRaycaster::new(grid)
}

fn convert_position(tp: &TilePosition) -> crisscross::TilePosition {
    crisscross::TilePosition::new(tp.col, tp.row, tp.rel_x, tp.rel_y)
}

/// A potential shot
#[derive(Debug)]
pub struct Shot {
    /// Direction in radians
    pub direction: f32,
    /// Relative distance from origin to target.
    /// This is the same as the global distance for tile size 1.
    pub distance: f32,
}

impl Display for Shot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Shot {{ distance: {}, angle: {} deg }}",
            self.distance, self.direction
        )
    }
}

#[allow(dead_code)]
pub fn find_shot(
    tc: &TileRaycaster,
    tile_path: &Tilepath,
    origin: &TilePosition,
    target: &TilePosition,
) -> Option<Shot> {
    let origin = convert_position(origin);
    let target = convert_position(target);
    // TODO(thlorenz): TilePosition.angle_to implementation got lost it seems and therefore we
    // hacked the angle just to be able to build for now
    let angle: AngleRad = (0.0).into(); // origin.angle_to(&target);
    let first_invalid = tc.first_invalid(&origin, angle.clone(), |tp| {
        if tp.is_same_tile(&target) {
            // hit the target
            false
        } else {
            // hit something else, i.e. a wall
            tile_path.is_valid(tp.x, tp.y)
        }
    })?;

    if first_invalid.is_same_tile(&target) {
        let distance = origin.distance_relative(&target);
        Some(Shot {
            direction: 0.0,
            distance,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::math::round;

    impl Shot {
        fn degrees(&self) -> Shot {
            Shot {
                direction: round(self.direction.to_degrees(), 1),
                distance: round(self.distance, 3),
            }
        }
    }

    impl PartialEq for Shot {
        fn eq(&self, other: &Self) -> bool {
            self.degrees() == other.degrees()
        }
    }

    macro_rules! assert_shot_eq {
        ($actual:expr, $expected:expr $(,)?) => {{
            let shot = $actual.expect("Expected some shot").degrees();
            assert_eq!(shot.distance, $expected.0);
            assert_eq!(shot.direction, $expected.1);
        }};
    }

    #[test]
    fn clear_shot() {
        let (ncols, nrows) = (4, 4);
        let tile_size = 1.0;
        let tile_path = Tilepath::all_valid(ncols as usize, nrows as usize);
        let tc = create_tile_caster(ncols, nrows, tile_size);

        let origin = TilePosition::new(0, 0, 0.5, 0.5);
        let target = TilePosition::new(1, 1, 0.5, 0.5);
        assert_shot_eq!(find_shot(&tc, &tile_path, &origin, &target), (1.414, 45.0));

        let origin = TilePosition::new(3, 3, 0.25, 0.25);
        let target = TilePosition::new(1, 1, 0.5, 0.5);
        assert_shot_eq!(find_shot(&tc, &tile_path, &origin, &target), (2.475, 225.0));
    }

    #[test]
    fn obstacled_shot() {
        let (ncols, nrows) = (8, 8);
        let tile_size = 1.0;
        let tile_path =
            Tilepath::with_invalids(ncols as usize, nrows as usize, vec![(0, 4), (4, 0), (6, 6)]);

        let tc = create_tile_caster(ncols, nrows, tile_size);

        let origin = TilePosition::new(0, 0, 0.5, 0.5);
        let target = TilePosition::new(1, 1, 0.5, 0.5);
        assert_shot_eq!(find_shot(&tc, &tile_path, &origin, &target), (1.414, 45.0));

        // target on same tile as obstacle is treated as reachable
        let target = TilePosition::new(0, 4, 0.5, 0.5);
        assert_shot_eq!(find_shot(&tc, &tile_path, &origin, &target), (4.0, 90.0));

        let target = TilePosition::new(0, 5, 0.5, 0.5);
        assert_eq!(find_shot(&tc, &tile_path, &origin, &target), None);

        let target = TilePosition::new(3, 0, 0.1, 0.1);
        assert_shot_eq!(find_shot(&tc, &tile_path, &origin, &target), (2.631, 351.3));

        let target = TilePosition::new(5, 0, 0.1, 0.1);
        assert_eq!(find_shot(&tc, &tile_path, &origin, &target), None);

        let target = TilePosition::new(5, 5, 0.9, 0.9);
        assert_shot_eq!(find_shot(&tc, &tile_path, &origin, &target), (7.637, 45.0));

        let target = TilePosition::new(7, 7, 0.9, 0.9);
        assert_eq!(find_shot(&tc, &tile_path, &origin, &target), None);

        let target = TilePosition::new(5, 7, 0.9, 0.9);
        assert_shot_eq!(find_shot(&tc, &tile_path, &origin, &target), (9.161, 53.9));
    }
}
