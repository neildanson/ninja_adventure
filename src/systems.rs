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

        if action.pressed(ControllerAction::RunLeft) && !action.pressed(ControllerAction::RunRight) {
            direction += Vec2::NEG_X;
        } else if action.pressed(ControllerAction::RunRight) && !action.pressed(ControllerAction::RunLeft) {
            direction += Vec2::X;
        }

        if action.pressed(ControllerAction::RunUp) && !action.pressed(ControllerAction::RunDown) {
            direction += Vec2::Y;
        } else if action.pressed(ControllerAction::RunDown) && !action.pressed(ControllerAction::RunUp) {
            direction += Vec2::NEG_Y;
        }
        velocity.linvel = direction * time.delta().as_secs_f32() * RUN_SPEED;

        state_logic(action, &mut state, PlayerState::RunRight(3), ControllerAction::RunRight, ControllerAction::RunLeft);
        state_logic(action, &mut state, PlayerState::RunLeft(2), ControllerAction::RunLeft, ControllerAction::RunRight);
        state_logic(action, &mut state, PlayerState::RunUp(1),  ControllerAction::RunUp, ControllerAction::RunDown);
        state_logic(action, &mut state, PlayerState::RunDown(0), ControllerAction::RunDown, ControllerAction::RunUp);

        if direction == Vec2::ZERO {
            *state = PlayerState::Idle;
        }

    }
}

fn state_logic(action: &ActionState<ControllerAction>, state: &mut PlayerState, new_state : PlayerState, action_to_test : ControllerAction, opposite_action : ControllerAction) {
    if action.get_just_released().len() == 0 && !action.pressed(opposite_action) {
        if action.just_pressed(action_to_test) {
            *state = new_state;
        } 
    } else {
        if action.pressed(action_to_test) {
            *state = new_state;
        } 
    }
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
