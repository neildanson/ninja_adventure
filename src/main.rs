use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use ninja_adventure::components::*;
use ninja_adventure::levels::*;
use ninja_adventure::plugins::*;
use ninja_adventure::systems::*;

fn startup(mut commands: Commands) {
    let mut cam = PixelCameraBundle::from_resolution(320, 240);
    cam.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 10.0));
    commands.spawn(cam);
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
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        //Leafless
        .add_plugin(InputManagerPlugin::<ControllerAction>::default())
        .add_plugin(PixelCameraPlugin)
        //Custom Debug Plugins
        .add_plugins(DebugPlugins)
        .add_loopless_state(GameState::InGame)
        .add_enter_system(GameState::InGame, level_startup)
        .add_system_set(ConditionSet::new().run_in_state(GameState::MainMenu).into())
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(spawn_obstacle_collision::<Tree>)
                .with_system(spawn_obstacle_collision::<Wall>)
                .with_system(player_input)
                .with_system(camera_follow)
                .with_system(animate)
                .into(),
        )
        .add_system_set(ConditionSet::new().run_in_state(GameState::GameOver).into())
        .register_ldtk_int_cell::<FloorBundle>(1)
        .register_ldtk_int_cell::<TreeBundle>(2)
        .register_ldtk_int_cell::<WallBundle>(3)
        .register_ldtk_int_cell::<WallBundle>(4)
        .register_ldtk_entity::<PlayerEntityBundle>("PlayerStart")
        .register_type::<PlayerState>()
        .run();
}
