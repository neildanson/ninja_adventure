use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn level_startup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ninja_adventure.ldtk"),
        ..Default::default()
    });

    commands.insert_resource(LevelSelection::Index(0));

    audio.play_with_settings(
        asset_server.load("Source/NinjaAdventure/Musics/1 - Adventure Begin.ogg"),
        PlaybackSettings::LOOP.with_volume(0.75),
    );
}
