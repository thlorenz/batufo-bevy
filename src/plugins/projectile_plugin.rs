use bevy::prelude::*;

use crate::{
    ecs::{
        components::{Hero, LifeCycle, Projectile, ProjectileSpawner, Velocity},
        events::ProjectileRequestedEvent,
    },
    engine::WorldPosition,
};

use super::game_plugin::GameRender;

#[derive(Default)]
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(stage::PRE_UPDATE, update_spawners.system())
            .add_system_to_stage(stage::POST_UPDATE, reset_spawners.system())
            .add_event::<ProjectileRequestedEvent>()
            .add_system(spawn_projectile.system())
            .add_system(detect_collision.system())
            .add_system(destroy_out_of_range_projectile.system());
    }
}

fn update_spawners(mut spawner_query: Query<&mut ProjectileSpawner>) {
    for mut spawner in spawner_query.iter_mut() {
        if spawner.ticks_until_reloaded > 0 {
            spawner.ticks_until_reloaded -= 1;
        }
    }
}

fn reset_spawners(mut spawner_query: Query<&mut ProjectileSpawner>) {
    for mut spawner in spawner_query.iter_mut() {
        if spawner.ticks_until_reloaded == 0 {
            spawner.ticks_until_reloaded = spawner.ticks_to_reload;
        }
    }
}

fn spawn_projectile(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event_reader: Local<EventReader<ProjectileRequestedEvent>>,
    game_render: Res<GameRender>,
    projectile_requested_events: Res<Events<ProjectileRequestedEvent>>,
) {
    let size = game_render.tile_size as f32 / 5.0;
    let mag = 0.2_f32;

    for event in event_reader.iter(&projectile_requested_events) {
        let mut velocity = Velocity::default();
        velocity.0.z = -mag * event.direction.sin();
        velocity.0.x = mag * event.direction.cos();

        let tp = &event.origin;
        let pos = tp.to_world_position(game_render.tile_size);

        // TODO(thlorenz): create the entire bundle or at least mesh and material once during setup
        // and add to resources
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(size, size, size))),
                material: {
                    let material = materials.add(Color::rgb(0.3, 0.4, 0.9).into());
                    material
                },
                transform: {
                    let transform: Transform = (&pos).into();
                    transform
                },
                ..Default::default()
            })
            .with(Projectile {
                origin: (&pos).into(),
                range: event.range * game_render.tile_size as f32,
                health_damage: event.health_damage,
            })
            .with(LifeCycle::default())
            .with(velocity);
    }
}

fn detect_collision(
    game_render: Res<GameRender>,
    mut projectile_query: Query<(&mut LifeCycle, &Transform, &Projectile)>,
    mut hero_query: Query<(&mut LifeCycle, &Transform), With<Hero>>,
) {
    // TODO(thlorenz): add TilePosition to Hero struct to calc it in one place once
    if let Some((mut hero_life, transform)) = hero_query.iter_mut().next() {
        let hero_tp = WorldPosition::from(transform).to_tile_position(game_render.tile_size);
        let hero_tp = match hero_tp {
            None => return,
            Some(hero_tp) => hero_tp,
        };
        for (mut life, transform, projectile) in projectile_query.iter_mut() {
            let tp = match WorldPosition::from(transform).to_tile_position(game_render.tile_size) {
                None => continue,
                Some(tp) => tp,
            };
            if tp.is_same_tile(&hero_tp) {
                hero_life.deduct(projectile.health_damage);
                println!("hero deducted health, now {}", hero_life.health());
                life.terminate();
            }
        }
    }
}

fn destroy_out_of_range_projectile(
    mut projectile_query: Query<(&mut LifeCycle, &Transform, &Projectile)>,
) {
    for (mut life, transform, projectile) in projectile_query.iter_mut() {
        let distance = projectile.origin.distance_squared(transform.translation);
        if distance >= projectile.range {
            life.terminate();
        }
    }
}
