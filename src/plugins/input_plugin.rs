use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use super::js_plugin::JsData;

use super::game_scene_plugin::Player;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(print_keyboard_event_system);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsEvent {
    pub event_type: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct KeyboardEventData {
    key_code: KeyCode,
}

fn print_keyboard_event_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    data: NonSend<Arc<Mutex<JsData>>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut js_events: Vec<JsEvent> = vec![];

    for event in keyboard_input_events.iter() {
        info!("{:?}", event);

        if let Some(key_code) = event.key_code {
            if keyboard_input.just_pressed(key_code) {
                let data = serde_json::to_string(&KeyboardEventData { key_code }).unwrap();
                js_events.push(JsEvent {
                    event_type: "keydown".into(),
                    data,
                });
            }
        }

        // for mut t in &mut query {
        //     t.translation += Vec3::new(0.0, 0.0, time.delta_seconds() * 1.0)
        // }
    }

    {
        let mut global_data = data.lock().unwrap();

        js_events.iter().for_each(|evt| {
            global_data.add_event(evt.clone());
        });

        // println!("data {:#?}", global_data.get_data())
    }
}
