use std::time::Duration;

use bevy::{ecs::prelude::*, prelude::*, utils::HashMap};

pub(super) struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (link_animations, start_animating, stop_animating),
        );
    }
}

/// An entity that is playing an animation.
#[derive(Clone, Component, Debug, Default)]
pub struct Animating {
    animation_clip_name: String,
}

impl Animating {
    pub fn from_animation_clip_name(name: String) -> Self {
        Self {
            animation_clip_name: name,
        }
    }
}

/// Stores [`AnimationClip`] references for a given glTF [`Scene`].
#[derive(Clone, Component, Debug, Default)]
pub struct AnimationClips(pub HashMap<String, Handle<AnimationClip>>);

/// Allows a parent entity to access the [`AnimationPlayer`] entity buried
/// within its [`Scene`] hierarchy.
#[derive(Component, Debug)]
pub struct AnimationEntityLink(pub Entity);

fn start_animating(
    mut query: Query<
        (&Animating, &AnimationClips, &AnimationEntityLink),
        Added<Animating>,
    >,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for (animating, animation_clips, animation_entity_link) in &mut query {
        if let Some(animation) =
            animation_clips.0.get(&animating.animation_clip_name)
        {
            if let Ok(mut animation_player) =
                animation_players.get_mut(animation_entity_link.0)
            {
                animation_player.resume();
                animation_player
                    .play_with_transition(
                        animation.clone_weak(),
                        Duration::from_secs(1),
                    )
                    .repeat();
            }
        }
    }
}

fn stop_animating(
    mut removed: RemovedComponents<Animating>,
    mut query: Query<&AnimationEntityLink>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for entity in removed.read() {
        if let Ok(animation_entity_link) = query.get_mut(entity) {
            if let Ok(mut animation_player) =
                animation_players.get_mut(animation_entity_link.0)
            {
                animation_player.pause();
            }
        }
    }
}

fn link_animations(
    animation_players_query: Query<Entity, Added<AnimationPlayer>>,
    all_entities_with_parents_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the hierarchy.
    for entity_with_animation_player in animation_players_query.iter() {
        let top_entity = get_top_parent(
            entity_with_animation_player,
            &all_entities_with_parents_query,
        );

        // If the top parent has an animation config ref then link the player to the config.
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animation players for the same top parent");
        } else {
            commands.entity(top_entity).insert(AnimationEntityLink(
                entity_with_animation_player.clone(),
            ));
        }
    }
}

fn get_top_parent(
    mut current_entity: Entity,
    all_entities_with_parents_query: &Query<&Parent>,
) -> Entity {
    // Loop up all the way to the top parent.
    loop {
        if let Ok(ref_to_parent) =
            all_entities_with_parents_query.get(current_entity)
        {
            current_entity = ref_to_parent.get();
        } else {
            break;
        }
    }
    current_entity
}