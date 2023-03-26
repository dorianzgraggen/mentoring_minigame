use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use super::game_scene_plugin::Player;

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
) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);

        // for mut t in &mut query {
        //     t.translation += Vec3::new(0.0, 0.0, time.delta_seconds() * 1.0)
        // }
    }
}
