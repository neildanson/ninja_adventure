use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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