use bevy::prelude::*;

use plugins::{
    commands_plugin::CommandsPlugin, game_scene_plugin::GameScenePlugin,
    global_data_plugin::GlobalDataPlugin, hello_plugin::HelloPlugin, input_plugin::InputPlugin,
    js_event_plugin::JsEventPlugin, js_plugin::JsPlugin,
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
    pub mod global_data_plugin;
    pub mod hello_plugin;
    pub mod input_plugin;
    pub mod js_event_plugin;
    pub mod js_plugin;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GlobalDataPlugin)
        // .add_plugins((GlobalDataPlugin, JsPlugin).chain())
        .configure_set(
            // Run systems in the Movement set before systems in the CollisionDetection set
            RunSet::Pre.before(RunSet::After),
        )
        // .add_plugin(HelloPlugin)
        .add_plugin(JsEventPlugin)
        .add_plugin(JsPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(GameScenePlugin)
        .add_plugin(CommandsPlugin)
        .run();
}
