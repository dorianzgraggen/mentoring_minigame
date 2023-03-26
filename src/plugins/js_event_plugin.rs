use bevy::prelude::*;

use std::net::UdpSocket;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use super::game_scene_plugin::Player;

pub struct JsEventPlugin;

impl Plugin for JsEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsEvent {
    pub event_type: String,
    pub data: String,
}

pub struct JsEventResource {
    events: Vec<JsEvent>,
}

impl JsEventResource {
    pub fn add_event(&mut self, event_type: String, data: String) {
        let event = JsEvent { event_type, data };
        self.events.push(event);

        // match self.tx.send(s) {
        //     Ok(_) => (),
        //     Err(error) => {
        //         panic!("error: {}", error);
        //     }
        // }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsEventList(Vec<JsEvent>);

fn setup(world: &mut World) {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let jer = JsEventResource { events: vec![] };
    let origin = Arc::new(Mutex::new(jer));

    let clone = Arc::clone(&origin);
    world.insert_non_send_resource(clone);

    std::thread::spawn(move || udp_server(rx, Arc::clone(&origin)));

    // let received = rx.recv().unwrap();
}

fn send_events_to_js(js_event_resource: NonSend<JsEventResource>) {}

fn udp_server(tx: Receiver<String>, jer: Arc<Mutex<JsEventResource>>) -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:52122")?;
    let mut buf = [0; 2048];

    println!("udp event running");

    loop {
        println!("waiting for request");
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..number_of_bytes];
        let s = std::str::from_utf8(buf).unwrap();

        println!("got help req: {}", s);

        // let new_list = jer
        //     .clone()
        //     .events
        //     .into_iter()
        //     .map(|e| JsEvent {
        //         event_type: e.event_type,
        //         data: e.data,
        //     })
        //     .collect();

        let response = {
            let jer2 = jer.lock().unwrap();
            let list = JsEventList(jer2.events.clone());
            serde_json::to_string(&list).unwrap()
        };

        let response = "loll";

        println!("will send {} to {}", response, src_addr);

        socket.send_to(response.as_bytes(), src_addr)?;
    }
}
