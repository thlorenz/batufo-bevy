use std::collections::HashMap;
use std::error::Error;

use crate::arena::builtins;
use crate::arena::tilemap::Tilemap;

pub struct Level {
    pub name: &'static str,
    pub terrain: &'static str,
}

impl Level {
    fn new(name: &'static str, terrain: &'static str) -> Level {
        Level { name, terrain }
    }
    #[allow(dead_code)]
    fn create_tilemap(&self, tile_size: u32) -> Result<Tilemap, Box<dyn Error>> {
        Tilemap::new(self.terrain, tile_size)
    }
}

pub struct Levels {
    levels: HashMap<&'static str, Level>,
}

impl Levels {
    pub fn new() -> Self {
        let mut levels: HashMap<&'static str, Level> = HashMap::new();
        add_level(
            &mut levels,
            "face off",
            builtins::face_off::level_face_off(),
        );
        add_level(
            &mut levels,
            "practice arena",
            builtins::practice_arena::level_practice_arena(),
        );
        add_level(&mut levels, "mini", builtins::mini::level_mini());
        Levels { levels }
    }

    #[allow(dead_code)]
    pub fn is_valid_level(&self, level_name: &'static str) -> bool {
        self.levels.contains_key(level_name)
    }

    pub fn get_level(&self, level_name: &'static str) -> Option<&Level> {
        self.levels.get(level_name)
    }
}

fn add_level(levels: &mut HashMap<&'static str, Level>, name: &'static str, terrain: &'static str) {
    levels.insert(name, Level::new(name, terrain));
}
