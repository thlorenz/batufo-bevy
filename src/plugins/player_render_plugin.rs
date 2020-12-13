use bevy::{asset::AssetPath, prelude::*};

use crate::{
    arena::arena::Arena,
    ecs::components::{HeadLights, Hero, Velocity},
};

use super::game_plugin::{GameAssets, GameRender};

#[derive(Default)]
pub struct PlayerRenderPlugin;

impl Plugin for PlayerRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_hero);
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
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.31)),
                    ..Default::default()
                })
                .with(HeadLights(true));
        })
        .with(Hero::default())
        .with(Velocity::default());
}
