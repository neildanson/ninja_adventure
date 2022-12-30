mod components;
mod constants;
mod levels;
mod plugins;
mod systems;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use components::*;
use constants::*;
use levels::*;
use plugins::*;
use systems::*;

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.25,
            ..default()
        },
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 800.0,
                        height: 600.0,
                        title: String::from("Ninja Adventure"),
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_startup_system(startup)
        //LDTK
        .add_plugin(LdtkPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        //Rapier2D
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(InputManagerPlugin::<Action>::default())
        //Custom Debug Plugins
        .add_plugins(DebugPlugins)
        .add_loopless_state(GameState::InGame)
        .add_enter_system(GameState::InGame, level_startup)
        .add_system_set(ConditionSet::new().run_in_state(GameState::MainMenu).into())
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(player_input)
                .with_system(camera_follow)
                .into(),
        )
        .add_system_set(ConditionSet::new().run_in_state(GameState::GameOver).into())
        .register_ldtk_int_cell::<FloorBundle>(1)
        .register_ldtk_entity::<PlayerEntityBundle>("PlayerStart")
        .run();
}
