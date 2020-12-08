use std::f32::consts::PI;

use bevy::{asset::AssetPath, prelude::*};

use crate::{
    arena::arena::Arena,
    ecs::components::{Hero, Velocity},
};

use super::game_plugin::GameProps;

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
    game_props: Res<GameProps>,
    arena: Res<Arena>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let asset = &game_props.assets.floor_tiles;
    let texture_handle = asset_server.load(AssetPath::new(asset.path.clone(), None));
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, asset.tile_size(), asset.rows, asset.cols);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let ntiles = asset.tiles();
    for (idx, tile) in arena.floor_tiles.iter().enumerate() {
        let sprite_idx = idx % ntiles;
        let pos = tile.to_world_position(game_props.render.tile_size);
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
            sprite: TextureAtlasSprite::new(sprite_idx as u32),
            ..Default::default()
        });
    }
}

fn setup_walls(
    commands: &mut Commands,
    game_props: Res<GameProps>,
    arena: Res<Arena>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let asset = &game_props.assets.wall_metal;
    let texture_handle = asset_server.load(AssetPath::new(asset.path.clone(), None));
    let material = materials.add(texture_handle.into());

    for tile in arena.walls.iter() {
        let pos = tile.to_world_position(game_props.render.tile_size);
        commands.spawn(SpriteBundle {
            material: material.clone(),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
            ..Default::default()
        });
    }
}

fn setup_hero(
    commands: &mut Commands,
    game_props: Res<GameProps>,
    arena: Res<Arena>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let asset = &game_props.assets.hero;
    let texture_handle = asset_server.load(AssetPath::new(asset.path.clone(), None));
    let material = materials.add(texture_handle.into());

    let pos = arena.player.to_world_position(game_props.render.tile_size);
    let mut transform = Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0));
    transform.scale = Vec3::splat(game_props.render.tile_size as f32 / asset.tile_size().x);
    transform.rotate(Quat::from_rotation_z(PI / 2.0));
    commands
        .spawn(SpriteBundle {
            material: material.clone(),
            transform,
            ..Default::default()
        })
        .with(Hero::default())
        .with(Velocity::default());
}
