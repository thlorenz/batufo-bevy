use bevy::prelude::*;

use crate::ecs::components::LifeCycle;

#[derive(Default)]
pub struct LifeCyclePlugin;

impl Plugin for LifeCyclePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(stage::POST_UPDATE, despawn_destroyed.system());
    }
}

fn despawn_destroyed(commands: &mut Commands, entities_query: Query<(Entity, &LifeCycle)>) {
    let need_despawn = entities_query.iter().filter(|(_, life)| !life.is_alive());
    for (entity, _) in need_despawn {
        commands.despawn_recursive(entity);
    }
}
