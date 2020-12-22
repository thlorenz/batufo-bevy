use std::{path::PathBuf, process};

use bevy::prelude::*;

use crate::{
    arena::{Arena, Tilepath},
    ecs::resources::PositionConverter,
};

pub struct GameRender {
    pub tile_size: u32,
    pub tile_render_size: f32,
}

pub struct GameAsset {
    pub width: f32,
    pub height: f32,
    pub rows: usize,
    pub cols: usize,
    pub path: PathBuf,
}

pub struct GameAssets {
    pub floor_tiles: GameAsset,
    pub wall_metal: GameAsset,
    pub hero: GameAsset,
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
            wall_metal: GameAsset {
                width: 48.0,
                height: 48.0,
                rows: 1,
                cols: 1,
                path: "textures/bg/wall-metal.png".into(),
            },
            hero: GameAsset {
                width: 192.0,
                height: 192.0,
                rows: 1,
                cols: 1,
                path: "textures/sprites/player.png".into(),
            },
        }
    }
}

pub struct GameCameras {
    pub platform_lerp: f32,
}

impl Default for GameCameras {
    fn default() -> Self {
        Self { platform_lerp: 1.0 }
    }
}

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let tile_size = 1;
        let render_scale = 0.1;
        let tile_render_size = render_scale * tile_size as f32;
        let render = GameRender {
            tile_size,
            tile_render_size,
        };

        let arena =
            Arena::for_level("face off", render.tile_size).expect("FATAL: unable to create arena");

        app.init_resource::<GameCameras>()
            .init_resource::<GameAssets>()
            .init_resource::<GameCameras>()
            .add_resource(render)
            .add_resource(PositionConverter::new(tile_size))
            .add_resource(Tilepath::from_arena(&arena))
            .add_resource(WindowDescriptor {
                title: "batufo".to_string(),
                width: 1024.0,
                height: 768.0,
                vsync: false,
                resizable: true,
                decorations: false,
                cursor_locked: false,
                cursor_visible: true,
                ..Default::default()
            })
            .add_resource(arena)
            .add_system(exit_game_system.system());
    }
}

fn exit_game_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        process::exit(0)
    }
}
