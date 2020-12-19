use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    animations::{Movement, MovementAxis, RollingBoxAnimation, RotationAxis, Spin},
    arena::arena::Arena,
    ecs::components::Hero,
    engine::position::TilePosition,
    plugins::path_finder_plugin::PathFinder,
};

use super::game_plugin::{GameAssets, GameRender};

enum GunTowerState {
    Idle,
    Moving(RollingBoxAnimation),
}

pub struct GunTower {
    step_factor: f32,
    center_y: f32,
    state: GunTowerState,
}

impl Default for GunTower {
    fn default() -> Self {
        GunTower {
            step_factor: 1.5,
            center_y: 0.5,
            state: GunTowerState::Idle,
        }
    }
}

#[derive(Default)]
pub struct GunTowerPlugin;

impl Plugin for GunTowerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(gun_tower_setup)
            .add_system(follow_hero_system);
    }
}

fn gun_tower_setup(
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

    pos.y = size * 0.6;

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(size, size, size))),
            material: {
                let material = materials.add(Color::rgb(0.9, 0.4, 0.2).into());
                material
            },
            transform: {
                let transform: Transform = (&pos).into();
                transform
            },
            ..Default::default()
        })
        .with(GunTower {
            center_y: (&pos).y.clone(),
            ..Default::default()
        });
}

fn follow_hero_system(
    time: Res<Time>,
    game_render: Res<GameRender>,
    pathfinder: Res<PathFinder>,
    mut tower_query: Query<(&mut Transform, &mut GunTower)>,
    hero_query: Query<&Transform, With<Hero>>,
) {
    for (mut tower_transform, mut gun_tower) in tower_query.iter_mut() {
        let step_factor = gun_tower.step_factor;
        match gun_tower.state {
            GunTowerState::Idle => {
                let hero_transform = hero_query.iter().next();
                if let Some(hero_transform) = hero_transform {
                    if let (tower_tile, Some(path)) = path_to_hero(
                        &pathfinder,
                        &tower_transform.translation,
                        &hero_transform.translation,
                    ) {
                        gun_tower.state = GunTowerState::Moving({
                            let (col, row) = path.first().map(|&(col, row)| (col, row)).unwrap();
                            let movement_axis =
                                MovementAxis::from_move_xz(tower_tile.col_row(), (col, row));
                            let rotation_axis = RotationAxis::from_movement_axis(&movement_axis);
                            let mut translation = pathfinder.translation_from_col_row((col, row));
                            translation.y = gun_tower.center_y;

                            RollingBoxAnimation {
                                movement: Movement::from_start_end(
                                    tower_transform.translation,
                                    translation,
                                    movement_axis,
                                ),
                                spin: Spin::from_delta_angle(
                                    &Quat::default(),
                                    rotation_axis,
                                    PI / 2.0,
                                ),
                                percent_complete: 0.0,
                            }
                        });
                    }
                }
            }
            GunTowerState::Moving(ref mut moving) => {
                let step_percent = time.delta_seconds() * step_factor;
                if moving.step_percent(&mut tower_transform, step_percent) {
                    gun_tower.state = GunTowerState::Idle;
                    let size = game_render.tile_size as f32;
                    tower_transform.translation.y = size * 0.6;
                }
            }
        };
    }
}

fn path_to_hero(
    pathfinder: &PathFinder,
    tower_pos: &Vec3,
    hero_pos: &Vec3,
) -> (TilePosition, Option<Vec<(u32, u32)>>) {
    let tower_tile = pathfinder
        .tile_from_translation(tower_pos)
        .expect("gun tower should never leave tilemap");
    let hero_tile = pathfinder.tile_from_translation(hero_pos);
    let path = hero_tile
        .map(|hero_tile| pathfinder.path(false, tower_tile.col_row(), hero_tile.col_row()))
        .flatten();
    (tower_tile, path)
}
