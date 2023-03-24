use bevy::prelude::*;

use plugins::{
    game_scene_plugin::GameScenePlugin, hello_plugin::HelloPlugin, input_plugin::InputPlugin,
    js_plugin::JsPlugin,
};

// TODO: wtf
mod plugins {
    pub mod game_scene_plugin;
    pub mod hello_plugin;
    pub mod input_plugin;
    pub mod js_plugin;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(HelloPlugin)
        .add_plugin(GameScenePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(JsPlugin)
        .run();
}
