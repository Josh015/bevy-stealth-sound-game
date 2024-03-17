use crate::common::constants::MOVEMENT_TOLERANCE;

use super::{rotating::Rotating, TurningSpeed};
use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

pub(super) struct TurningPlugin;

impl Plugin for TurningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (start_turning_to, turning_to).chain());
    }
}

/// Rotates a [`TurningSpeed`] entity to a new rotation before removing itself.
#[derive(Clone, Component, Debug, new)]
pub struct TurningTo {
    direction: Direction3d,
}

fn start_turning_to(
    mut commands: Commands,
    query: Query<
        (Entity, &TurningSpeed, &TurningTo, &Transform),
        Added<TurningTo>,
    >,
) {
    for (entity, turning_speed, turning_to, transform) in &query {
        commands.entity(entity).insert(Rotating::new(
            Direction3d::new_unchecked(
                (*transform.forward())
                    .cross(*turning_to.direction)
                    .normalize(),
            ),
            turning_speed.0,
        ));
    }
}

fn turning_to(
    mut commands: Commands,
    query: Query<(Entity, &TurningTo, &Transform)>,
) {
    for (entity, turning_to, transform) in &query {
        if (*transform.forward()).dot(*turning_to.direction).abs()
            >= 1.0 - MOVEMENT_TOLERANCE
        {
            commands.entity(entity).remove::<(Rotating, TurningTo)>();
        }
    }
}
