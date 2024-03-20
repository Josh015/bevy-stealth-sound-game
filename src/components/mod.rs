// TODO: These all need to act as physical attributes, status conditions,
// senses, event emitters, and data storage. Don’t have them perform AI logic!

use bevy::app::prelude::*;

mod actors;
mod mover;
mod physics;
mod status_effects;
mod trigger;

pub use actors::*;
pub use mover::*;
pub use physics::*;
pub use status_effects::*;
pub use trigger::*;

pub(super) struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AgentsPlugin, MoverPlugin, PhysicsPlugin));
    }
}
