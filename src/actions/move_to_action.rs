use bevy::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

use crate::Destination;

/// Move an entity to a point.
///
/// **WARNING:** Can't be used in parallel with [`TurnToAction`](super::turn_to_action::TurnToAction).
#[derive(new)]
pub struct MoveToAction {
    destination: Vec3,
}

impl Action for MoveToAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<Destination>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world
            .entity_mut(agent)
            .insert(Destination(self.destination));
        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<Destination>();
    }
}