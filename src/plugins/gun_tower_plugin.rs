use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    ai::find_path,
    animations::{
        Movement, MovementAnimation, MovementAxis, RollingBoxAnimation, RotationAxis, Spin,
    },
    arena::{Arena, Tilepath},
    ecs::{
        components::{Hero, HeroFollower, MovementState, OrthogonalMovement, ProjectileSpawner},
        resources::{PositionConverter, Sniper},
    },
    engine::TilePosition,
};

use super::game_plugin::{GameAssets, GameRender};

#[derive(Default)]
pub struct GunTowerPlugin;

impl Plugin for GunTowerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(gun_tower_setup.system())
            .add_system(follow_hero_system.system())
            .add_system(shoot_hero_system.system());
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
        .with(OrthogonalMovement {
            center_y: (&pos).y.clone(),
            ..Default::default()
        })
        .with(HeroFollower)
        .with(ProjectileSpawner { range: 15.0 });
}

fn shoot_hero_system(
    sniper: Res<Sniper>,
    tilepath: Res<Tilepath>,
    shooter_query: Query<(&Transform, &ProjectileSpawner), With<HeroFollower>>,
    hero_query: Query<&Transform, With<Hero>>,
) {
    for (transform, spawner) in shooter_query.iter() {
        let hero_transform = hero_query.iter().next().unwrap();
        let shot = sniper.find_shot(&tilepath, &transform, &hero_transform, Some(spawner.range));
        if let Some(shot) = shot {
            println!("shot: {:#?}", shot);
        }
    }
}

fn follow_hero_system(
    time: Res<Time>,
    game_render: Res<GameRender>,
    converter: Res<PositionConverter>,
    tilepath: Res<Tilepath>,
    mut follower_query: Query<(&mut Transform, &mut OrthogonalMovement), With<HeroFollower>>,
    hero_query: Query<&Transform, With<Hero>>,
) {
    for (mut follower_transform, mut follower_movement) in follower_query.iter_mut() {
        let step_factor = follower_movement.step_factor;
        match follower_movement.state {
            MovementState::Idle => {
                let hero_transform = hero_query.iter().next();
                if let Some(hero_transform) = hero_transform {
                    if let (tower_tile, Some(path)) = path_to_hero(
                        &converter,
                        &tilepath,
                        &follower_transform.translation,
                        &hero_transform.translation,
                    ) {
                        follower_movement.state = MovementState::Moving({
                            let (col, row) = path.first().map(|&(col, row)| (col, row)).unwrap();
                            let movement_axis =
                                MovementAxis::from_move_xz(tower_tile.col_row(), (col, row));
                            let rotation_axis = RotationAxis::from_movement_axis(&movement_axis);
                            let mut translation = converter.translation_from_col_row((col, row));
                            translation.y = follower_movement.center_y;

                            let rolling_box_animation = RollingBoxAnimation {
                                movement: Movement::from_start_end(
                                    follower_transform.translation,
                                    translation,
                                    movement_axis,
                                ),
                                spin: Spin::from_delta_angle(
                                    &Quat::default(),
                                    rotation_axis,
                                    PI / 2.0,
                                ),
                                percent_complete: 0.0,
                            };
                            MovementAnimation::Rolling(rolling_box_animation)
                        });
                    }
                }
            }
            MovementState::Moving(ref mut movement) => {
                let step_percent = time.delta_seconds() * step_factor;
                match movement {
                    MovementAnimation::Rolling(rolling) => {
                        if rolling.step_percent(&mut follower_transform, step_percent) {
                            follower_movement.state = MovementState::Idle;
                            let size = game_render.tile_size as f32;
                            follower_transform.translation.y = size * 0.6;
                        }
                    }
                }
            }
        };
    }
}

fn path_to_hero(
    converter: &PositionConverter,
    tilepath: &Tilepath,
    tower_pos: &Vec3,
    hero_pos: &Vec3,
) -> (TilePosition, Option<Vec<(u32, u32)>>) {
    let tower_tile = converter
        .tile_from_translation(tower_pos)
        .expect("gun tower should never leave tilemap");

    let hero_tile = converter.tile_from_translation(hero_pos);

    let path = hero_tile
        .map(|hero_tile| {
            find_path(
                &tilepath.valid_tiles,
                false,
                tower_tile.col_row(),
                hero_tile.col_row(),
            )
        })
        .flatten();

    (tower_tile, path)
}
