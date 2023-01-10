use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::components::*;
use crate::constants::*;

pub fn level_startup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(LEVEL_FILE),
        ..Default::default()
    });

    commands.insert_resource(LevelSelection::Index(0));

    audio.play_with_settings(
        asset_server.load(LEVEL_MUSIC),
        PlaybackSettings::LOOP.with_volume(0.75),
    );
}

pub fn camera_follow(
    mut is_set: Local<bool>,
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    camera: Query<Entity, With<Camera>>,
) {
    if !*is_set {
        for p in player.iter() {
            for c in camera.iter() {
                let player = commands.entity(p).id();
                let camera = commands.entity(c).id();
                commands.entity(player).push_children(&[camera]);

                *is_set = true;
            }
        }
    }
}
pub fn player_input(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Velocity,
            &mut PlayerState,
            &ActionState<ControllerAction>,
        ),
        With<Player>,
    >,
) {
    for (mut velocity, mut state, action) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if action.pressed(ControllerAction::RunLeft) {
            direction += Vec2::NEG_X;
        } else if action.pressed(ControllerAction::RunRight) {
            direction += Vec2::X;
        }

        if action.pressed(ControllerAction::RunUp) {
            direction += Vec2::Y;
        } else if action.pressed(ControllerAction::RunDown) {
            direction += Vec2::NEG_Y;
        }
        velocity.linvel = direction * time.delta().as_secs_f32() * RUN_SPEED;

        if action.just_released(ControllerAction::RunDown)
            || action.just_released(ControllerAction::RunUp)
            || action.just_released(ControllerAction::RunLeft)
            || action.just_released(ControllerAction::RunRight)
        {
            *state = pressed(action);
        }

        *state = just_pressed(action, *state);
    }
}

fn pressed(action: &ActionState<ControllerAction>) -> PlayerState {
    if action.pressed(ControllerAction::RunUp) {
        return PlayerState::RunUp(1);
    } else if action.pressed(ControllerAction::RunDown) {
        return PlayerState::RunDown(0);
    }
    if action.pressed(ControllerAction::RunLeft) {
        return PlayerState::RunLeft(2);
    } else if action.pressed(ControllerAction::RunRight) {
        return PlayerState::RunRight(3);
    }
    return PlayerState::Idle;
}

fn just_pressed(action: &ActionState<ControllerAction>, state: PlayerState) -> PlayerState {
    if action.just_pressed(ControllerAction::RunUp) {
        return PlayerState::RunUp(1);
    } else if action.just_pressed(ControllerAction::RunDown) {
        return PlayerState::RunDown(0);
    }

    if action.just_pressed(ControllerAction::RunLeft) {
        return PlayerState::RunLeft(2);
    } else if action.just_pressed(ControllerAction::RunRight) {
        return PlayerState::RunRight(3);
    }

    return state;
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(
        &mut PlayerState,
        &mut TextureAtlasSprite,
        &mut AnimationTimer,
    )>,
) {
    for (mut state, mut sprite, mut timer) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            *state = state.update();
        }
        sprite.index = state.frame();
    }
}
