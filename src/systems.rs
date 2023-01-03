use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::components::*;
use crate::constants::*;

pub fn camera_follow(
    player: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let mut camera = camera.single_mut();

    for player in player.iter() {
        camera.translation = Vec3::new(
            player.translation.x,
            player.translation.y,
            camera.translation.z,
        );
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
        //If it is pressed then increment the frame and check bounds
        //If nothing is pressed, default to idle
        let mut new_state = None;
        if action.pressed(ControllerAction::RunLeft) {
            if action.just_pressed(ControllerAction::RunLeft) {
                new_state = Some(PlayerState::RunLeft(2));
            } else {
                let frame = (state.frame() + 4) % ANIM_FRAMES;

                new_state = Some(PlayerState::RunLeft(frame));
            }
        }

        if action.pressed(ControllerAction::RunRight) {
            if action.just_pressed(ControllerAction::RunRight) {
                new_state = Some(PlayerState::RunRight(3));
            } else {
                let frame = (state.frame() + 4) % ANIM_FRAMES;

                new_state = Some(PlayerState::RunRight(frame));
            }
        }

        if action.pressed(ControllerAction::RunUp) {
            if action.just_pressed(ControllerAction::RunUp) {
                new_state = Some(PlayerState::RunUp(1));
            } else {
                let frame = (state.frame() + 4) % ANIM_FRAMES;

                new_state = Some(PlayerState::RunUp(frame));
            }
        }

        if action.pressed(ControllerAction::RunDown) {
            if action.just_pressed(ControllerAction::RunDown) {
                new_state = Some(PlayerState::RunDown(0));
            } else {
                let frame = (state.frame() + 4) % ANIM_FRAMES;

                new_state = Some(PlayerState::RunDown(frame));
            }
        }


        *state = new_state.unwrap_or(PlayerState::Idle);
    }
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&PlayerState, &mut TextureAtlasSprite, &mut AnimationTimer)>,
) {
    for (state, mut sprite, mut timer) in query.iter_mut() {
        timer.tick(time.delta());
        sprite.index = if timer.just_finished() {
            state.frame()
        } else {
            sprite.index
        }
    }
}
