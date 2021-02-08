use std::path::PathBuf;

use bevy::prelude::*;

use crate::{
    arena::Arena,
    ecs::components::{HeadLights, Hero, LifeCycle, Velocity},
};

use super::game_plugin::{GameAssets, GameRender};

#[derive(Default)]
pub struct PlayerRenderPlugin;

impl Plugin for PlayerRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_hero.system());
    }
}

fn setup_hero(
    commands: &mut Commands,
    game_assets: Res<GameAssets>,
    game_render: Res<GameRender>,
    arena: Res<Arena>,
    asset_server: Res<AssetServer>,
) {
    let mut pos = arena.player.to_world_position(game_render.tile_size);
    let size = game_render.tile_size as f32;
    pos.y = size * 0.2;

    commands
        .spawn((
            {
                let mut transform: Transform = pos.into();
                transform.scale = transform.scale * 0.35;
                transform
            },
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load(PathBuf::from(&game_assets.hero.path)));
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
        .with(LifeCycle::default())
        .with(Velocity::default());
}
