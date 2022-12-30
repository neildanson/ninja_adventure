use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

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
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
) {
    for mut controller in query.iter_mut() {
        let mut direction: Option<Vec2> = None;
        if keys.pressed(KeyCode::Left) {
            direction = update_translation(direction, Vec2::NEG_X);
        } else if keys.pressed(KeyCode::Right) {
            direction = update_translation(direction, Vec2::X);
        }

        if keys.pressed(KeyCode::Up) {
            direction = update_translation(direction, Vec2::Y);
        } else if keys.pressed(KeyCode::Down) {
            direction = update_translation(direction, Vec2::NEG_Y);
        }
        controller.translation = direction;
    }
}
