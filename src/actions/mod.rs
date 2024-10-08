use bevy::prelude::*;
use bevy_sequential_actions::*;

use crate::prelude::*;

mod action_sequence;
mod animation_action;
mod emote_action;
mod face_direction_action;
mod move_to_action;
mod parallel_actions;
mod repeat_action;
mod repeat_sequence;
mod sound_action;
mod wait_action;

pub use action_sequence::*;
pub use animation_action::*;
pub use emote_action::*;
pub use face_direction_action::*;
pub use move_to_action::*;
pub use parallel_actions::*;
pub use repeat_action::*;
pub use repeat_sequence::*;
pub use sound_action::*;
pub use wait_action::*;

pub(super) struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), pause_all_action_queues)
            .add_systems(OnExit(GameState::Paused), resume_all_action_queues)
            .add_systems(OnEnter(GameState::GameOver), pause_all_action_queues)
            .add_systems(OnExit(GameState::GameOver), resume_all_action_queues)
            .add_systems(
                Update,
                pause_added_action_queues.in_set(ActiveWhenPausedSet),
            )
            .add_plugins(WaitActionPlugin);
    }
}

fn pause_all_action_queues(
    mut commands: Commands,
    query: Query<Entity, With<ActionQueue>>,
) {
    for entity in &query {
        commands.actions(entity).pause();
    }
}

fn resume_all_action_queues(
    mut commands: Commands,
    query: Query<Entity, With<ActionQueue>>,
) {
    for entity in &query {
        commands.actions(entity).execute();
    }
}

fn pause_added_action_queues(
    mut commands: Commands,
    query: Query<Entity, Added<ActionQueue>>,
) {
    for entity in &query {
        commands.actions(entity).pause();
    }
}
