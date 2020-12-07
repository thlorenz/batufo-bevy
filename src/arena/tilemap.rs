use std::cmp::max;
use std::error::Error;
use std::fmt;
use std::vec::Vec;

const EMPTY: char = ' ';

#[derive(Clone)]
pub enum Tile {
    OutOfBounds = 0,
    Empty = 1,
    Hole = 2,
    Wall = 3,
    Player = 4,
    Medkit = 5,
    Shield = 6,
    Bomb = 7,
    Teleport1 = 8,
    Teleport2 = 9,
    Teleport3 = 10,
    Teleport4 = 11,
    Teleport5 = 12,
    Teleport6 = 13,
    Teleport7 = 14,
    Teleport8 = 15,
}

pub fn needs_floor_tile(tile: &Tile) -> bool {
    match tile {
        Tile::OutOfBounds | Tile::Wall | Tile::Hole => false,
        Tile::Empty
        | Tile::Player
        | Tile::Shield
        | Tile::Medkit
        | Tile::Bomb
        | Tile::Teleport1
        | Tile::Teleport2
        | Tile::Teleport3
        | Tile::Teleport4
        | Tile::Teleport5
        | Tile::Teleport6
        | Tile::Teleport7
        | Tile::Teleport8 => true,
    }
}

pub struct Tilemap {
    pub tile_size: u32,
    pub tiles: Vec<Tile>,
    pub nrows: u32,
    pub ncols: u32,
}

fn get_bounds(row: &str) -> (usize, usize) {
    let start = row.find('=').unwrap_or(0);
    let end = row.rfind('=').unwrap_or(start);
    let end = if end == start { row.len() - 1 } else { end };
    (start, end)
}

fn tile_from_char(c: char) -> Result<Tile, String> {
    match c {
        EMPTY => Ok(Tile::Empty),
        'x' => Ok(Tile::Hole),
        'p' => Ok(Tile::Player),
        '=' => Ok(Tile::Wall),
        '+' => Ok(Tile::Medkit),
        's' => Ok(Tile::Shield),
        'b' => Ok(Tile::Bomb),
        '1' => Ok(Tile::Teleport1),
        '2' => Ok(Tile::Teleport2),
        '3' => Ok(Tile::Teleport3),
        '4' => Ok(Tile::Teleport4),
        '5' => Ok(Tile::Teleport5),
        '6' => Ok(Tile::Teleport6),
        '7' => Ok(Tile::Teleport7),
        '8' => Ok(Tile::Teleport8),
        _ => Err(format!("Unknown char {}", c)),
    }
}

fn char_from_tile(tile: &Tile) -> char {
    match tile {
        Tile::OutOfBounds => 'X',
        Tile::Empty => ' ',
        Tile::Hole => 'x',
        Tile::Wall => '=',
        Tile::Player => 'p',
        Tile::Medkit => '+',
        Tile::Shield => 's',
        Tile::Bomb => 'b',
        Tile::Teleport1 => '1',
        Tile::Teleport2 => '2',
        Tile::Teleport3 => '3',
        Tile::Teleport4 => '4',
        Tile::Teleport5 => '5',
        Tile::Teleport6 => '6',
        Tile::Teleport7 => '7',
        Tile::Teleport8 => '8',
    }
}

impl Tilemap {
    pub fn new(terrain: &str, tile_size: u32) -> Result<Self, Box<dyn Error>> {
        let lines: Vec<&str> = terrain
            .lines()
            .skip_while(|s| s.trim().is_empty())
            .take_while(|s| !s.trim().is_empty())
            .collect();

        let nrows = lines.len();
        let ncols = lines.iter().fold(0, |acc, s| max(acc, s.len()));
        let ntiles = nrows * ncols;

        let mut tiles: Vec<Tile> = vec![Tile::OutOfBounds; ntiles];

        #[allow(clippy::needless_range_loop)]
        for row in 0..nrows {
            let line = lines[row];
            let (start, end) = get_bounds(line);
            let tile_row = nrows - row - 1;
            let mut col: usize = start;
            for c in line.chars().into_iter().skip(start).take(end - start + 1) {
                let idx = tile_row * ncols + col;
                let tile = tile_from_char(c)?;
                tiles[idx] = tile;
                col = col + 1;
            }
        }

        let _start = 0;

        Ok(Tilemap {
            tile_size,
            tiles,
            nrows: nrows as u32,
            ncols: ncols as u32,
        })
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for row in self.tiles.chunks(self.ncols as usize).rev() {
            for tile in row {
                let c = char_from_tile(tile);
                s.push(c);
            }
            s.push('\n');
        }
        s
    }
}

impl fmt::Debug for Tilemap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
Tilemap {{
  nrows: {nrows},
  ncols: {ncols}
{to_string}
}}",
            nrows = self.nrows,
            ncols = self.ncols,
            to_string = self.to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_terrain() {
        let terrain = "


=======================
=         p           =
=                     =
=====           p =====
=     ====        =
=     =  =        =
=     ====        =
=====              =======
=   +   p       p        =
=                     ====
=======================

";
        let tilemap = Tilemap::new(terrain, 24).expect("should produce a tilemap");
        print!("tilemap {:?}", tilemap);
    }
}
