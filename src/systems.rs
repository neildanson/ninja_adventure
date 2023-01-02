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
pub fn player_input(
    mut query: Query<
        (
            &mut Velocity,
            &ActionState<ControllerAction>,
        ),
        With<Player>,
    >,
) {
    for (mut velocity, action) in query.iter_mut() {
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
        velocity.linvel = direction * 60.0;
    }
}


pub fn animate(
    time: Res<Time>, mut query: Query<(&mut TextureAtlasSprite,  &mut AnimationTimer)>) {
        for (mut sprite, mut timer) in query.iter_mut() {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = (sprite.index + 4) % 16;
            }
        }
    }