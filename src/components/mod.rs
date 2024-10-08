use bevy::app::prelude::*;

mod animations_clips;
mod barrier;
mod door;
mod drop_shadow;
mod emote;
mod floor_switch;
mod footsteps;
mod for_state;
mod glass;
mod guard;
mod movement;
mod physics;
mod pickup;
mod player;
mod security_camera;
mod speed;
mod weapon;

pub use animations_clips::*;
pub use barrier::*;
pub use door::*;
pub use drop_shadow::*;
pub use emote::*;
pub use floor_switch::*;
pub use footsteps::*;
pub use for_state::*;
pub use glass::*;
pub use guard::*;
pub use movement::*;
pub use physics::*;
pub use pickup::*;
pub use player::*;
pub use security_camera::*;
pub use speed::*;
pub use weapon::*;

pub(super) struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AnimationClipsPlugin,
            FootstepsPlugin,
            ForStatePlugin,
            GuardPlugin,
            MovementPlugin,
            PhysicsPlugin,
            PickupPlugin,
            PlayerPlugin,
        ));
    }
}
