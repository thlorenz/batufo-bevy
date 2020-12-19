use pathfinding::prelude::bfs;

pub fn find_path(
    valid_tiles: &Vec<Vec<bool>>,
    allow_diagonals: bool,
    start: (u32, u32),
    end: (u32, u32),
) -> Option<Vec<(u32, u32)>> {
    let result = bfs(
        &start,
        |&p| moves(&valid_tiles, allow_diagonals, p),
        |&p| p == end,
    );
    result
        .map(|tiles| tiles.into_iter().skip(1).collect())
        .filter(|tiles: &Vec<_>| !tiles.is_empty())
}

fn moves(valid_tiles: &Vec<Vec<bool>>, allow_diagonals: bool, tile: (u32, u32)) -> Vec<(u32, u32)> {
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
        .filter(|&(col, row)| valid_tiles[col as usize][row as usize])
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::arena::{Arena, Tilepath};

    fn init_tile_path_for_face_off() -> Tilepath {
        let tile_size = 1;
        let arena = Arena::for_level("face off", tile_size).expect("FATAL: unable to create arena");
        Tilepath::from_arena(&arena)
    }

    #[test]
    fn find_path_1() {
        let tilepath = init_tile_path_for_face_off();
        let path = find_path(&tilepath.valid_tiles, false, (11, 19), (11, 21));
        let expected: Vec<(u32, u32)> = vec![(11, 20), (11, 21)];
        assert_eq!(path, Some(expected));
    }

    #[test]
    fn find_path_2() {
        let tilepath = init_tile_path_for_face_off();
        let path = find_path(&tilepath.valid_tiles, false, (5, 25), (62, 25));
        assert_eq!(path.map(|x| x.len()), Some(71));
    }
}
