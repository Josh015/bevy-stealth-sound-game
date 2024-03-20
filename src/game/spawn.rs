use bevy::{ecs::prelude::*, prelude::*, utils::HashMap};
use spew::prelude::*;

use crate::{
    AnimationClips, Barrier, BlocksVision, DeflectsSounds, Door, DropShadow,
    GuardBundle, Hearing, MoverBundle, Pickup, Player, SecurityCamera,
    Shatterable, Stunnable, Vision, Weapon,
};

use super::{ActorConfig, ComponentConfig};

pub(super) struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SpewPlugin::<Config, (Handle<ActorConfig>, Vec3)>::default(),
        ))
        .add_spawner((Config::Actor, spawn_actor_from_config_at_position));
    }
}

/// Entities that can be spawned from config files.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Config {
    Actor,
    Level,
    SoundWave,
}

fn spawn_actor_from_config_at_position(
    In((handle, position)): In<(Handle<ActorConfig>, Vec3)>,
    actor_configs: Res<Assets<ActorConfig>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let actor_config = actor_configs.get(handle).unwrap();
    let mut actor = commands.spawn_empty();

    for component_config in &actor_config.components {
        match component_config {
            ComponentConfig::Player => {
                actor.insert(Player::default());
            },
            ComponentConfig::Guard => {
                actor.insert(GuardBundle::default());
            },
            ComponentConfig::SecurityCamera => {
                actor.insert(SecurityCamera::default());
            },
            ComponentConfig::Pickup => {
                actor.insert(Pickup::default());
            },
            ComponentConfig::Weapon => {
                actor.insert(Weapon::default());
            },
            ComponentConfig::Powerup { file_name } => {
                // TODO: Need a component for this one.
            },
            //Trigger {} // TODO: Probably want to have a sub-enum with pre-allowed events?
            ComponentConfig::Door => {
                actor.insert(Door::default());
            },
            ComponentConfig::Barrier => {
                actor.insert(Barrier::default());
            },
            ComponentConfig::Mover => {
                // TODO: Implement setting Linear and Angular Speed?
                actor.insert(MoverBundle::default());
            },
            ComponentConfig::Physics { radius } => {
                // TODO: Need a component for this one.
            },
            ComponentConfig::Footsteps { sound_wave } => {
                // TODO: Need a component for this one.
            },
            ComponentConfig::DropShadow => {
                actor.insert(DropShadow::default());
            },
            ComponentConfig::Vision => {
                // TODO: Implement setting the fields.
                actor.insert(Vision::default());
            },
            ComponentConfig::Hearing => {
                // TODO: Implement setting the fields.
                actor.insert(Hearing::default());
            },
            ComponentConfig::Stunnable => {
                actor.insert(Stunnable::default());
            },
            ComponentConfig::BlocksVision => {
                actor.insert(BlocksVision::default());
            },
            ComponentConfig::DeflectsSounds => {
                actor.insert(DeflectsSounds::default());
            },
            ComponentConfig::Shatterable => {
                actor.insert(Shatterable::default());
            },
            ComponentConfig::AnimationClips(clips) => {
                // TODO: Load animations and store indices.
                actor.insert(AnimationClips(HashMap::default()));
            },
            ComponentConfig::Scene(scene) => {
                actor.insert(SceneBundle {
                    scene: asset_server.load(scene),
                    transform: Transform::from_matrix(
                        Mat4::from_scale_rotation_translation(
                            Vec3::splat(0.0025), // TODO: Integrate scale into config?
                            Quat::from_rotation_y(std::f32::consts::PI),
                            position,
                        ),
                    ),
                    ..default()
                });
            },
        }
    }
}