use bevy::{app::PluginGroupBuilder, prelude::*, window::close_on_esc};
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app : &mut App) {
        app.add_system(close_on_esc);
    }
}

pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        if cfg!(debug_assertions) {
            let group = PluginGroupBuilder::start::<Self>();
            group
                .add(DebugPlugin)
                .add(WorldInspectorPlugin::new())
                .add(RapierDebugRenderPlugin::default())
        } else {
            PluginGroupBuilder::start::<Self>()
        }
    }
}
