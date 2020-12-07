use bevy::{asset::AssetPath, prelude::*};

use crate::arena::arena::Arena;

use super::game_plugin::GameProps;

#[derive(Default)]
pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_arena);
    }
}

fn setup_arena(
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
            texture_atlas: texture_atlas_handle.clone_weak(),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
            sprite: TextureAtlasSprite::new(sprite_idx as u32),
            ..Default::default()
        });
    }
}
