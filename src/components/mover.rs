use bevy::prelude::*;

pub const FORWARD_DIRECTION: Vec3 = Vec3::NEG_Z;
pub const MOTION_MARGIN_OF_ERROR: f32 = 0.01;

pub(super) struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (linear_velocity, angular_velocity, mover).chain(),
        );
    }
}

/// Linear velocity that updates translation over time.
#[derive(Clone, Component, Debug)]
pub struct LinearVelocity(pub Vec3);

/// Angular velocity which updates rotation over time.
#[derive(Clone, Component, Debug)]
pub struct AngularVelocity {
    pub axis: Direction3d,
    pub velocity: f32,
}

/// Linear speed in `meters/second`.
#[derive(Clone, Component, Debug)]
pub struct LinearSpeed(pub f32);

impl Default for LinearSpeed {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Angular speed in `radians/second`.
#[derive(Clone, Component, Debug)]
pub struct AngularSpeed(pub f32);

impl Default for AngularSpeed {
    fn default() -> Self {
        Self(std::f32::consts::TAU)
    }
}

/// Specify what type of movement is required.
#[derive(Clone, Copy, Debug)]
pub enum MoveTo {
    Destination(Vec3),
    Direction(Direction3d),
}

/// Moves an entity.
#[derive(Clone, Component, Debug, Default)]
pub struct Mover {
    move_to: Option<MoveTo>,
    heading: Option<Vec3>,
}

impl Mover {
    pub fn start(&mut self, move_to: MoveTo) {
        self.move_to = Some(move_to);
        self.heading = None;
    }

    pub fn stop(&mut self) {
        self.move_to = None;
    }

    pub fn is_finished(&self) -> bool {
        self.move_to.is_none() && self.heading.is_none()
    }
}

/// Required components for [`Mover`] to work.
#[derive(Bundle, Clone, Debug, Default)]
pub struct MoverBundle {
    pub mover: Mover,
    pub linear_speed: LinearSpeed,
    pub angular_speed: AngularSpeed,
}

fn linear_velocity(
    time: Res<Time>,
    mut query: Query<(&LinearVelocity, &mut Transform)>,
) {
    for (linear_velocity, mut transform) in &mut query {
        transform.translation += linear_velocity.0 * time.delta_seconds();
    }
}

fn angular_velocity(
    time: Res<Time>,
    mut query: Query<(&AngularVelocity, &mut Transform)>,
) {
    for (angular_velocity, mut transform) in &mut query {
        transform.rotation = (transform.rotation
            * Quat::from_axis_angle(
                *angular_velocity.axis,
                angular_velocity.velocity * time.delta_seconds(),
            ))
        .normalize();
    }
}

fn mover(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Mover,
        &LinearSpeed,
        &AngularSpeed,
        &mut Transform,
        Has<LinearVelocity>,
        Has<AngularVelocity>,
    )>,
) {
    for (
        entity,
        mut mover,
        linear_speed,
        angular_speed,
        mut transform,
        has_linear_velocity,
        has_angular_velocity,
    ) in &mut query
    {
        match (mover.move_to, mover.heading) {
            // Initialize and cache data before inserting working components.
            (Some(move_to), None) => {
                let mut entity = commands.entity(entity);
                let heading = match move_to {
                    MoveTo::Destination(destination) => {
                        let heading =
                            (destination - transform.translation).normalize();

                        entity.insert(LinearVelocity(heading * linear_speed.0));
                        heading
                    },
                    MoveTo::Direction(heading) => {
                        // A precaution in case mode was switched midway.
                        if has_linear_velocity {
                            entity.remove::<LinearVelocity>();
                        }

                        *heading
                    },
                };

                mover.heading = Some(heading);
                entity.insert(AngularVelocity {
                    axis: Direction3d::new_unchecked(
                        (*transform.forward()).cross(heading).normalize(),
                    ),
                    velocity: angular_speed.0,
                });
            },

            // Check progress and eventually remove the working components.
            (Some(move_to), Some(heading)) => {
                let mut entity = commands.entity(entity);

                if let MoveTo::Destination(destination) = move_to {
                    if has_linear_velocity
                        && destination.distance(transform.translation)
                            <= MOTION_MARGIN_OF_ERROR
                    {
                        entity.remove::<LinearVelocity>();
                        transform.translation = destination;
                    }
                }

                if has_angular_velocity
                    && transform.forward().dot(heading).abs()
                        >= 1.0 - MOTION_MARGIN_OF_ERROR
                {
                    entity.remove::<AngularVelocity>();
                }

                if !has_linear_velocity && !has_angular_velocity {
                    mover.move_to = None;
                    mover.heading = None;
                }
            },

            // Reset to default state if mover was stopped externally.
            (None, Some(_)) => {
                mover.heading = None;

                if has_linear_velocity || has_angular_velocity {
                    commands
                        .entity(entity)
                        .remove::<(AngularVelocity, LinearVelocity)>();
                }
            },
            _ => {},
        }
    }
}