use bevy::prelude::*;

use std::net::UdpSocket;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(poll);
    }
}

pub struct CommandsResource {
    rx: Receiver<String>,
}

fn setup(world: &mut World) {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    world.insert_non_send_resource(CommandsResource { rx });
    std::thread::spawn(move || udp_server(tx));

    // let received = rx.recv().unwrap();
}

fn poll(cr: NonSend<CommandsResource>) {
    if let Ok(received) = cr.rx.try_recv() {
        println!("polled: {}", received);
    }
}

fn udp_server(tx: Sender<String>) -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:2000")?;
    let mut buf = [0; 2048];

    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..number_of_bytes];
        let s = std::str::from_utf8(buf).unwrap();
        println!("received: {}", s);

        match tx.send(s.into()) {
            Ok(_) => (),
            Err(error) => {
                panic!("error: {}", error);
            }
        }
    }
}
