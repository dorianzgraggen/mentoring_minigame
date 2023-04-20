use std::net::SocketAddr;
use std::net::UdpSocket;
use std::rc::Rc;

use bevy::prelude::*;

use deno_core::error::AnyError;
use deno_core::include_js_files;
use deno_core::op;
use deno_core::Extension;
use deno_core::OpState;

use tokio::time::{sleep, Duration};

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::RunSet;

use super::input_plugin::JsEvent;

pub struct JsPlugin;

impl Plugin for JsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_js_system.in_set(RunSet::Pre));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsData {
    events: Vec<JsEvent>,
}

impl JsData {
    pub fn add_event(&mut self, data: JsEvent) {
        self.events.push(data);
    }

    pub fn get_events(&self) -> Vec<JsEvent> {
        self.events.clone()
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}

fn spawn_js_system(world: &mut World) {
    let js_data = Arc::new(Mutex::new(JsData { events: vec![] }));
    world.insert_non_send_resource(Arc::clone(&js_data));
    std::thread::spawn(move || js_system(Arc::clone(&js_data)));
}

fn js_system(data: Arc<Mutex<JsData>>) {
    let file_path = "./app.js";

    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_name("js_plugin thread")
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_js(file_path, data)) {
        eprintln!("error: {error}");
    }
}

#[op]
async fn op_fetch(url: String) -> Result<String, AnyError> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[op]
fn op_get_str(state: &mut OpState, key: String) -> Result<String, AnyError> {
    let s = state.borrow_mut::<OpStateData>().test_value.clone();
    Ok(s)
}

#[op]
fn op_command(state: &mut OpState, command: String) -> Result<(), AnyError> {
    state.borrow_mut::<OpStateData>().write_command(command);
    Ok(())
}

#[op]
fn op_get_events_json(state: &mut OpState) -> Result<String, AnyError> {
    let events = state.borrow_mut::<OpStateData>().get_events();
    Ok(events)
}

#[op]
async fn op_sleep(milliseconds: u64) -> Result<(), AnyError> {
    sleep(Duration::from_millis(milliseconds)).await;
    Ok(())
}

async fn run_js(file_path: &str, data: Arc<Mutex<JsData>>) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;
    let runjs_extension = Extension::builder("runjs")
        .esm(include_js_files!("js_plugin_runtime.js",))
        .ops(vec![
            op_fetch::decl(),
            op_get_str::decl(),
            op_command::decl(),
            op_sleep::decl(),
            op_get_events_json::decl(),
        ])
        .build();
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });

    let op_state = js_runtime.op_state();

    let sock = UdpSocket::bind("0.0.0.0:8080").unwrap();

    op_state.borrow_mut().put(OpStateData {
        test_value: "Hallo i bims".to_string(),
        udp_socket: sock,
        js_data: data,
    });

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

struct OpStateData {
    pub test_value: String,
    pub udp_socket: UdpSocket,
    pub js_data: Arc<Mutex<JsData>>,
}

impl OpStateData {
    // TODO: https://tokio.rs/tokio/topics/bridging
    pub fn write_command(&mut self, command: String) {
        let addr = SocketAddr::from(([127, 0, 0, 1], 2000));
        let bytes = command.as_bytes();
        self.udp_socket.send_to(bytes, addr).unwrap();
    }

    // TODO: find out how deno does this
    pub fn get_events(&mut self) -> String {
        // let addr = SocketAddr::from(([127, 0, 0, 1], 52122));
        // let bytes = "send help".as_bytes();
        // self.sock.send_to(bytes, addr).unwrap();

        // let mut buf = [0; 2048];
        // let (number_of_bytes, src_addr) = self.sock.recv_from(&mut buf).unwrap();

        // let filled_buf = &mut buf[..number_of_bytes];

        let stringified = {
            let mut s = self.js_data.lock().unwrap();
            let data = s.get_events();
            let stringified = serde_json::to_string(&data).unwrap();
            s.clear_events();
            stringified
        };

        std::str::from_utf8(stringified.as_bytes()).unwrap().into()
    }
}
