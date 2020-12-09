use bevy::{asset::AssetPath, prelude::*};

use crate::{
    arena::arena::Arena,
    ecs::components::{HeadLights, Hero, Velocity},
};

use super::game_plugin::{GameAssets, GameRender};

#[derive(Default)]
pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_floor)
            .add_startup_system(setup_walls)
            .add_startup_system(setup_hero);
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
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(size, size / 10.0, size))),
            material: material.clone(),
            transform,
            ..Default::default()
        });
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

fn setup_hero(
    commands: &mut Commands,
    game_assets: Res<GameAssets>,
    game_render: Res<GameRender>,
    arena: Res<Arena>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let asset = &game_assets.hero;
    let texture_handle = asset_server.load(AssetPath::new(asset.path.clone(), None));
    let material = materials.add(texture_handle.into());

    let mut pos = arena.player.to_world_position(game_render.tile_size);
    let size = game_render.tile_size as f32;
    pos.y = size * 0.8;
    let transform: Transform = pos.into();

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(size, size / 4.0, size * 1.5))),
            material,
            transform,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(LightBundle {
                    light: Light {
                        color: Color::rgb_linear(1.0, 1.0, 0.8),
                        depth: 0.1..5.0,
                        fov: f32::to_radians(15.0),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, -15.0)),
                    ..Default::default()
                })
                .with(HeadLights(true));
        })
        .with(Hero::default())
        .with(Velocity::default());
}
