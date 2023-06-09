use bevy::prelude::*;

use std::net::UdpSocket;
use std::sync::mpsc::{self, Receiver, Sender};

use serde::{Deserialize, Serialize};

use super::game_scene_plugin::Player;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(poll);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    id: String,
    args: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArrayF32(Vec<f32>);

pub struct CommandsResource {
    rx: Receiver<String>,
}

fn setup(world: &mut World) {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    world.insert_non_send_resource(CommandsResource { rx });
    std::thread::spawn(move || udp_server(tx));

    // let received = rx.recv().unwrap();
}

fn poll(
    cr: NonSend<CommandsResource>,
    mut query: Query<&mut Transform, With<Player>>,
    players: Query<&Player>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(received) = cr.rx.try_recv() {
        let deserialized: Command = serde_json::from_str(&received).unwrap();

        let command_id = deserialized.id.as_str();
        match command_id {
            "player_set_position" => {
                let args: ArrayF32 = serde_json::from_str(&deserialized.args).unwrap();
                for mut t in &mut query {
                    t.translation = Vec3::new(args.0[0], args.0[1], args.0[2]);
                }
            }
            "player_set_color" => {
                let args: ArrayF32 = serde_json::from_str(&deserialized.args).unwrap();
                for p in &players {
                    if let Some(material) = materials.get_mut(&p.material) {
                        material.base_color = Color::hsl(args.0[0], args.0[1], args.0[2]);
                    }
                }
            }
            _ => {
                println!("command not found: {}", command_id);
            }
        }
    }
}

fn udp_server(tx: Sender<String>) -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:2000")?;
    let mut buf = [0; 2048];

    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..number_of_bytes];
        let s = std::str::from_utf8(buf).unwrap();

        match tx.send(s.into()) {
            Ok(_) => (),
            Err(error) => {
                panic!("error: {}", error);
            }
        }
    }
}
