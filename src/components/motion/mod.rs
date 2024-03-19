use bevy::prelude::*;

mod angular_velocity;
mod mover;
mod speed;
mod velocity;

pub use angular_velocity::*;
pub use mover::*;
pub use speed::*;
pub use velocity::*;

pub(super) struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AngularVelocityPlugin, MoverPlugin, VelocityPlugin));
    }
}
