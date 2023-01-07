use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::components::*;
use crate::constants::*;

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

        //If something is just pressed, set the state to its 1st frame
        //If nothing is pressed, default to idle
        let mut new_state = None;

        if action.pressed(ControllerAction::RunDown) {
            if action.just_pressed(ControllerAction::RunDown) {
                new_state = Some(PlayerState::RunDown(0));
            } else {
                new_state = Some(state.clone());
            }
        }

        if action.pressed(ControllerAction::RunUp) {
            if action.just_pressed(ControllerAction::RunUp) {
                new_state = Some(PlayerState::RunUp(1));
            } else {
                new_state = Some(state.clone());
            }
        }

        if action.pressed(ControllerAction::RunLeft) {
            if action.just_pressed(ControllerAction::RunLeft) {
                new_state = Some(PlayerState::RunLeft(2));
            } else {
                new_state = Some(state.clone());
            }
        }

        if action.pressed(ControllerAction::RunRight) {
            if action.just_pressed(ControllerAction::RunRight) {
                new_state = Some(PlayerState::RunRight(3));
            } else {
                new_state = Some(state.clone());
            }
        }

        *state = new_state.unwrap_or(PlayerState::Idle);
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
