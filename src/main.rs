mod constants;
mod plugins;
mod components;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use components::*;
use plugins::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1280.0,
                        height: 1024.0,
                        title: String::from("Ninja Adventure"),
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_loopless_state(GameState::InGame)
        .add_system_set(ConditionSet::new().run_in_state(GameState::MainMenu).into())
        .add_system_set(ConditionSet::new().run_in_state(GameState::GameOver).into())
        .add_plugin(LdtkPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(DebugPlugins)
        .run();
}
