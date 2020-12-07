use std::path::PathBuf;

use bevy::prelude::*;

use crate::arena::arena::Arena;

pub struct GameRender {
    pub tile_size: u32,
}

pub struct GameAsset {
    pub width: f32,
    pub height: f32,
    pub rows: usize,
    pub cols: usize,
    pub path: PathBuf,
}

impl GameAsset {
    pub fn tile_size(&self) -> Vec2 {
        Vec2::new(
            self.width / self.rows as f32,
            self.height / self.cols as f32,
        )
    }

    pub fn tiles(&self) -> usize {
        self.rows * self.cols
    }
}

pub struct GameAssets {
    pub floor_tiles: GameAsset,
}

impl Default for GameAssets {
    fn default() -> Self {
        Self {
            floor_tiles: GameAsset {
                width: 384.0,
                height: 384.0,
                rows: 8,
                cols: 8,
                path: "textures/bg/floor-tiles.png".into(),
            },
        }
    }
}

pub struct GameProps {
    pub render: GameRender,
    pub assets: GameAssets,
}

impl GameProps {
    #[allow(dead_code)]
    pub fn default() -> Self {
        let tile_size = 48;
        let render = GameRender { tile_size };
        Self {
            render,
            assets: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let game_props = GameProps::default();
        let arena = Arena::for_level("face off", game_props.render.tile_size)
            .expect("FATAL: unable to create arena");
        app.add_resource(game_props).add_resource(arena);
    }
}
