use bevy::prelude::*;

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::RunSet;

use super::js_plugin::GlobalData;

pub struct GlobalDataPlugin;

impl Plugin for GlobalDataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalDataChangeTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .add_startup_system(setup.in_set(RunSet::After))
        .add_system(change_data);
    }
}

#[derive(Resource)]
struct GlobalDataChangeTimer(Timer);

fn setup(world: &mut World) {}

fn change_data(
    time: Res<Time>,
    mut timer: ResMut<GlobalDataChangeTimer>,
    data: NonSend<Arc<Mutex<GlobalData>>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut global_data = data.lock().unwrap();
        global_data.add_data("miau".into());
        println!("data {:#?}", global_data.get_data())
    }
}
