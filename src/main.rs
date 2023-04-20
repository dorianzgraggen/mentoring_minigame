use bevy::prelude::*;

use plugins::{
    commands_plugin::CommandsPlugin, game_scene_plugin::GameScenePlugin, input_plugin::InputPlugin,
    js_plugin::JsPlugin,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum RunSet {
    Pre,
    After,
}

// TODO: wtf
mod plugins {
    pub mod commands_plugin;
    pub mod game_scene_plugin;
    pub mod input_plugin;
    pub mod js_plugin;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .configure_set(RunSet::Pre.before(RunSet::After))
        .add_plugin(JsPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(GameScenePlugin)
        .add_plugin(CommandsPlugin)
        .run();
}
