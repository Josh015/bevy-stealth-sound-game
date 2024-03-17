use crate::common::constants::MOVEMENT_TOLERANCE;

use super::{translating::Translating, MovingSpeed};
use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

pub(super) struct MovingToPlugin;

impl Plugin for MovingToPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (start_moving_to, moving_to).chain());
    }
}

/// Moves a [`MovingSpeed`] entity to a new position before removing itself.
#[derive(Clone, Component, Debug, new)]
pub struct MovingTo {
    position: Vec3,
}

fn start_moving_to(
    mut commands: Commands,
    query: Query<
        (Entity, &MovingSpeed, &MovingTo, &Transform),
        Added<MovingTo>,
    >,
) {
    for (entity, moving_speed, moving_to, transform) in &query {
        commands.entity(entity).insert(Translating::new(
            (moving_to.position - transform.translation).normalize_or_zero()
                * moving_speed.0,
        ));
    }
}

fn moving_to(
    mut commands: Commands,
    query: Query<(Entity, &MovingTo, &Transform)>,
) {
    for (entity, moving_to, transform) in &query {
        if moving_to.position.distance(transform.translation)
            <= MOVEMENT_TOLERANCE
        {
            commands.entity(entity).remove::<(Translating, MovingTo)>();
        }
    }
}
