use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::components::*;

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

fn update_translation(t: Option<Vec2>, d: Vec2) -> Option<Vec2> {
    match t {
        Some(t) => Some(t + d),
        None => Some(d),
    }
}

pub fn player_input(
    mut query: Query<(&mut KinematicCharacterController, &ActionState<Action>), With<Player>>,
) {
    for (mut controller, action) in query.iter_mut() {
        let mut direction: Option<Vec2> = None;
        if action.pressed(Action::RunLeft) {
            direction = update_translation(direction, Vec2::NEG_X);
        } else if action.pressed(Action::RunRight) {
            direction = update_translation(direction, Vec2::X);
        }

        if action.pressed(Action::RunUp){
            direction = update_translation(direction, Vec2::Y);
        } else if action.pressed(Action::RunDown) {
            direction = update_translation(direction, Vec2::NEG_Y);
        }
        controller.translation = direction;
    }
}
