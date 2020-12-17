use bevy::prelude::*;

use crate::engine::position::TilePosition;
use crate::plugins::path_finder_plugin::PathFinder;
use crate::{arena::arena::Arena, ecs::components::Hero};

use super::game_plugin::{GameAssets, GameRender};
use crate::plugins::gun_tower_plugin::GunTowerState::Idle;

const GUN_TOWER_MOVE_INTERVAL_SEC: f32 = 1.0;

enum GunTowerState {
    Idle,
    MovingTo(Vec3),
}

impl Default for GunTowerState {
    fn default() -> Self {
        Idle
    }
}

#[derive(Default)]
pub struct GunTower {
    state: GunTowerState,
}

#[derive(Default)]
pub struct GunTowerPlugin;

impl Plugin for GunTowerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_gun_tower)
            .add_system(tick_gun_tower);
    }
}

fn setup_gun_tower(
    commands: &mut Commands,
    _game_assets: Res<GameAssets>,
    game_render: Res<GameRender>,
    _arena: Res<Arena>,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut pos = TilePosition::centered(20, 20, game_render.tile_size)
        .to_world_position(game_render.tile_size);
    let size = game_render.tile_size as f32;
    pos.y = size * 0.5;

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(size, size, size))),
            material: {
                let material = materials.add(Color::rgb(0.9, 0.4, 0.2).into());
                material
            },
            transform: {
                let transform: Transform = pos.into();
                transform
            },
            ..Default::default()
        })
        .with(GunTower::default())
        .with(Timer::from_seconds(GUN_TOWER_MOVE_INTERVAL_SEC, true));
}

fn tick_gun_tower(
    time: Res<Time>,
    pathfinder: Res<PathFinder>,
    mut tower_query: Query<(&mut Timer, &mut Transform), With<GunTower>>,
    hero_query: Query<&Transform, With<Hero>>,
) {
    for (mut timer, mut tower_transform) in tower_query.iter_mut() {
        if timer.tick(time.delta_seconds()).just_finished() {
            let current_tile = pathfinder.tile_from_translation(&tower_transform.translation);
            for transform in hero_query.iter() {
                let hero_tile = pathfinder.tile_from_translation(&transform.translation);
                let path_to_player =
                    pathfinder.path(false, current_tile.col_row(), hero_tile.col_row());
                if let Some(path) = path_to_player {
                    let y = tower_transform.translation.y;
                    tower_transform.translation = {
                        let mut translation =
                            pathfinder.translation_from_col_row(*path.first().unwrap());
                        translation.y = y;
                        translation
                    };
                } else {
                    println!("cannot find path from {} to {}", &current_tile, &hero_tile);
                }
            }
        }
    }
}
