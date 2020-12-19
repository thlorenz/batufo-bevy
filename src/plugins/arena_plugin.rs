use bevy::{asset::AssetPath, prelude::*};
use bevy_mod_picking::*;

use crate::{arena::Arena, ecs::components::FloorTile};

use super::game_plugin::{GameAssets, GameRender};

#[derive(Default)]
pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_floor)
            .add_startup_system(setup_walls);
    }
}

fn setup_floor(
    commands: &mut Commands,
    game_assets: Res<GameAssets>,
    game_render: Res<GameRender>,
    arena: Res<Arena>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO: this is actually a spritesheet and the floor tiles should map to once tile
    // in it, instead of rendering the entire thing, however haven't found a way to get
    // at individual textures of a texture atlas.
    let asset = &game_assets.floor_tiles;
    let texture_handle = asset_server.load(AssetPath::new(asset.path.clone(), None));
    let material = materials.add(texture_handle.into());
    let size = game_render.tile_size as f32 * 0.92;

    for (_idx, tile) in arena.floor_tiles.iter().enumerate() {
        let pos = tile.to_world_position(game_render.tile_size);
        let transform: Transform = pos.into();
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(size, size / 10.0, size))),
                material: material.clone(),
                transform,
                ..Default::default()
            })
            .with(PickableMesh::default())
            .with(FloorTile(tile.clone()));
    }
}

fn setup_walls(
    commands: &mut Commands,
    game_assets: Res<GameAssets>,
    game_render: Res<GameRender>,
    arena: Res<Arena>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let asset = &game_assets.wall_metal;
    let texture_handle = asset_server.load(AssetPath::new(asset.path.clone(), None));
    let material = materials.add(texture_handle.into());

    let size = game_render.tile_size as f32 * 0.95;

    for tile in arena.walls.iter() {
        let mut pos = tile.to_world_position(game_render.tile_size);
        pos.y = size * 0.5;
        let transform: Transform = pos.into();
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(size, size, size))),
            material: material.clone(),
            transform,
            ..Default::default()
        });
    }
}
