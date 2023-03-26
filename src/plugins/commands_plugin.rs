use bevy::prelude::*;

use std::net::UdpSocket;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup() {
    std::thread::spawn(udp_server);
}

fn udp_server() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:2000")?;
    let mut buf = [0; 2048];

    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..number_of_bytes];
        let str = std::str::from_utf8(buf).unwrap();
        println!("received: {}", str);
    }
}
