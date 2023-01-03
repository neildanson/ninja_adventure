use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Floor;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct FloorBundle {
    wall: Floor,
}

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum ControllerAction {
    RunLeft,
    RunRight,
    RunUp,
    RunDown,
    Attack,
    Interact,
}

#[derive(Component, Default, Clone)]
pub enum PlayerState {
    #[default]
    Idle,
    RunLeft(usize),
    RunRight(usize),
    RunUp(usize),
    RunDown(usize),
}

impl PlayerState {
    pub fn frame(&self) -> usize {
        match self {
            PlayerState::Idle => 0,
            PlayerState::RunLeft(f) => *f,
            PlayerState::RunRight(f) => *f,
            PlayerState::RunUp(f) => *f,
            PlayerState::RunDown(f) => *f,
        }
    }
}

#[derive(Component, Deref, DerefMut, Default, Clone)]
pub struct AnimationTimer(pub Timer);

#[derive(Clone, Bundle, LdtkIntCell, Default)]
pub struct PlayerBundle {
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
    pub animation_timer: AnimationTimer,
    pub action_state: ActionState<ControllerAction>,
    pub input_map: InputMap<ControllerAction>,
    pub player_state: PlayerState,
}

impl From<EntityInstance> for PlayerBundle {
    fn from(entity_instance: EntityInstance) -> PlayerBundle {
        match entity_instance.identifier.as_ref() {
            "PlayerStart" => PlayerBundle {
                rigid_body: RigidBody::Dynamic,
                velocity: Velocity::default(),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                collider: Collider::cuboid(PLAYER_WIDTH / 2.0, PLAYER_HEIGHT / 2.0),
                animation_timer: AnimationTimer(Timer::from_seconds(ANIM_TIMER, TimerMode::Repeating)),
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::Up, ControllerAction::RunUp),
                    (KeyCode::Left, ControllerAction::RunLeft),
                    (KeyCode::Down, ControllerAction::RunDown),
                    (KeyCode::Right, ControllerAction::RunRight),
                    (KeyCode::Space, ControllerAction::Attack),
                    (KeyCode::Return, ControllerAction::Interact),
                ]),
                ..default()
            },

            _ => PlayerBundle::default(),
        }
    }
}

#[derive(Clone, Bundle, LdtkEntity, Default)]
pub struct PlayerEntityBundle {
    #[from_entity_instance]
    #[bundle]
    pub player_bundle: PlayerBundle,

    #[sprite_sheet_bundle(
        "Source/NinjaAdventure/Actor/Characters/GreenNinja/SpriteSheet.png",
        16.0,
        16.0,
        4,
        7,
        0.0,
        0.0,
        0
    )]
    #[bundle]
    pub sprite: SpriteSheetBundle,

    #[worldly]
    pub worldly: Worldly,

    pub player: Player,
}
