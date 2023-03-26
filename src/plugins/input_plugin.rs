use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use super::js_event_plugin::JsEvent;

use super::game_scene_plugin::Player;
use super::js_event_plugin::JsEventResource;
use std::sync::{Arc, Mutex};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(print_keyboard_event_system);
    }
}

fn print_keyboard_event_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    js_event_resource: NonSend<Arc<Mutex<JsEventResource>>>,
) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);

        {
            let mut jer = js_event_resource.lock().unwrap();
            jer.add_event("keydown".into(), "lölll".into());
        }

        // for mut t in &mut query {
        //     t.translation += Vec3::new(0.0, 0.0, time.delta_seconds() * 1.0)
        // }
    }
}
