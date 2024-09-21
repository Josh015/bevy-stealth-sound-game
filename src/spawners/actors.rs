use crate::{assets::*, components::*, game_state::*};
use bevy::{
    ecs::{prelude::*, system::SystemState},
    prelude::*,
    utils::HashMap,
};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

pub(super) struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<ActorConfig>::new(&["actor.ron"]))
            .observe(spawn_actor_from_config_with_matrix);
    }
}

#[derive(Event)]
pub enum SpawnActor {
    WithTransform(Handle<ActorConfig>, Mat4),
}

/// Actor entity configuration.
#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct ActorConfig(pub Vec<ActorProperty>);

/// Properties for configuring actor entities.
///
/// Don't necessarily map 1:1 to the entity's components.
#[derive(Clone, Debug, Deserialize)]
pub enum ActorProperty {
    Player,
    Guard,
    SecurityCamera,
    Pickup,
    Weapon,
    //Trigger {} // Probably want to have a sub-enum with pre-allowed events?
    FloorSwitch,
    Door,
    Glass,
    Speed {
        linear_speed: f32,
        angular_speed: f32,
    },
    Physics {
        radius: f32,
    },
    Footsteps {
        sound_wave: String,
    },
    DropShadow,
    Vision,
    Hearing,
    Stunnable,
    Barrier,
    BlocksVision,
    DeflectsSounds,
    Scene(String),
    AnimationClips(HashMap<String, String>),
}

/// Assets that need to be loaded in advance of spawning entities.
#[derive(Debug, Resource)]
pub struct PreloadedActorAssets {
    pub scenes: HashMap<String, Handle<Scene>>,
    pub animation_clips: HashMap<String, Handle<AnimationClip>>,
}

impl FromWorld for PreloadedActorAssets {
    fn from_world(world: &mut World) -> Self {
        let mut system_state: SystemState<(
            Res<AssetServer>,
            Res<GameAssets>,
            Res<Assets<ActorConfig>>,
        )> = SystemState::new(world);
        let (asset_server, game_assets, actor_config_assets) =
            system_state.get_mut(world);
        let mut scenes: HashMap<String, Handle<Scene>> = HashMap::default();
        let mut animation_clips: HashMap<String, Handle<AnimationClip>> =
            HashMap::default();

        for (_, actor) in &game_assets.actors {
            let Some(actor) = actor_config_assets.get(actor) else {
                continue;
            };

            // Preload all referenced assets in entity configs.
            for property in &actor.0 {
                match property {
                    ActorProperty::Scene(path) => {
                        if scenes.get(path).is_none() {
                            scenes.insert(
                                path.to_string(),
                                asset_server.load(path),
                            );
                        }
                    },
                    ActorProperty::AnimationClips(mappings) => {
                        for (_, path) in mappings {
                            if animation_clips.get(path).is_none() {
                                animation_clips.insert(
                                    path.to_string(),
                                    asset_server.load(path),
                                );
                            }
                        }
                    },
                    _ => {},
                }
            }
        }

        Self {
            scenes,
            animation_clips,
        }
    }
}

fn spawn_actor_from_config_with_matrix(
    trigger: Trigger<SpawnActor>,
    actor_configs: Res<Assets<ActorConfig>>,
    game_assets: Res<GameAssets>,
    mut commands: Commands,
    preloaded_actor_assets: Res<PreloadedActorAssets>,
) {
    let (handle, matrix) = match trigger.event() {
        SpawnActor::WithTransform(handle, matrix) => (handle, matrix),
    };

    let actor_config = actor_configs.get(handle).unwrap();
    let mut entity_commands = commands
        .spawn(ForStates(vec![GameState::Gameplay, GameState::GameOver]));

    for property in &actor_config.0 {
        match property {
            ActorProperty::Player => {
                entity_commands.insert(PlayerBundle::default());
            },
            ActorProperty::Guard => {
                entity_commands.insert(GuardBundle::default());
            },
            ActorProperty::SecurityCamera => {
                entity_commands.insert(SecurityCameraBundle::default());
            },
            ActorProperty::Pickup => {
                entity_commands.insert(PickupBundle::default());
            },
            ActorProperty::Weapon => {
                entity_commands.insert(Weapon::default());
            },
            //Trigger {} // TODO: Probably want to have a sub-enum with
            // pre-allowed events?
            ActorProperty::FloorSwitch => {
                entity_commands.insert(FloorSwitchBundle::default());
            },
            ActorProperty::Door => {
                entity_commands.insert(DoorBundle::default());
            },
            ActorProperty::Glass => {
                entity_commands.insert(GlassBundle::default());
            },
            ActorProperty::Speed {
                linear_speed,
                angular_speed,
            } => {
                entity_commands.insert(SpeedBundle {
                    linear_speed: LinearSpeed(*linear_speed),
                    angular_speed: AngularSpeed(*angular_speed),
                    ..default()
                });
            },
            ActorProperty::Physics { radius } => {
                // TODO: Need a component for this one.
            },
            ActorProperty::Footsteps { sound_wave } => {
                let sound_wave_handle =
                    game_assets.sound_waves.get(sound_wave.as_str()).unwrap();

                entity_commands.insert(FootstepsBundle {
                    footsteps: Footsteps {
                        sound_wave: sound_wave_handle.clone(),
                    },
                });
            },
            ActorProperty::DropShadow => {
                entity_commands.insert(DropShadow::default());
            },
            ActorProperty::Vision => {
                // TODO: Implement setting the fields.
                entity_commands.insert(Vision::default());
            },
            ActorProperty::Hearing => {
                // TODO: Implement setting the fields.
                entity_commands.insert(Hearing::default());
            },
            ActorProperty::Stunnable => {
                entity_commands.insert(Stunnable::default());
            },
            ActorProperty::Barrier => {
                entity_commands.insert(Barrier::default());
            },
            ActorProperty::BlocksVision => {
                entity_commands.insert(BlocksVision::default());
            },
            ActorProperty::DeflectsSounds => {
                entity_commands.insert(DeflectsSounds::default());
            },
            ActorProperty::AnimationClips(clips) => {
                let mut loaded_clips = HashMap::default();

                for (k, v) in clips {
                    loaded_clips.insert(
                        k.to_string(),
                        preloaded_actor_assets
                            .animation_clips
                            .get(v)
                            .unwrap()
                            .clone(),
                    );
                }

                entity_commands.insert(AnimationClips(loaded_clips));
            },
            ActorProperty::Scene(scene) => {
                entity_commands.insert(SceneBundle {
                    scene: preloaded_actor_assets
                        .scenes
                        .get(scene)
                        .unwrap()
                        .clone(),
                    transform: Transform::from_matrix(*matrix),
                    ..default()
                });
            },
        }
    }
}
