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

pub struct JsPlugin;

impl Plugin for JsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_js_system.in_set(RunSet::Pre));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalData {
    events: Vec<String>,
}

impl GlobalData {
    pub fn add_data(&mut self, data: String) {
        self.events.push(data);
    }

    pub fn get_data(&self) -> Vec<String> {
        self.events.clone()
    }
}

fn spawn_js_system(world: &mut World) {
    let origin = Arc::new(Mutex::new(GlobalData { events: vec![] }));
    let clone = Arc::clone(&origin);
    world.insert_non_send_resource(clone);

    let clone2 = Arc::clone(&origin);

    std::thread::spawn(move || js_system(clone2));
}

fn js_system(data: Arc<Mutex<GlobalData>>) {
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
    let s = state.borrow_mut::<Commander>().value.clone();
    Ok(s)
}

#[op]
fn op_command(state: &mut OpState, command: String) -> Result<(), AnyError> {
    state.borrow_mut::<Commander>().write_command(command);
    Ok(())
}

#[op]
fn op_get_events_json(state: &mut OpState) -> Result<String, AnyError> {
    let events = state.borrow_mut::<Commander>().get_events();
    Ok(events)
}

#[op]
async fn op_sleep(milliseconds: u64) -> Result<(), AnyError> {
    sleep(Duration::from_millis(milliseconds)).await;
    Ok(())
}

async fn run_js(file_path: &str, data: Arc<Mutex<GlobalData>>) -> Result<(), AnyError> {
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

    op_state.borrow_mut().put(Commander {
        value: "Hallo i bims".to_string(),
        sock,
        data,
    });

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

struct Commander {
    pub value: String,
    pub sock: UdpSocket,
    pub data: Arc<Mutex<GlobalData>>,
}

impl Commander {
    // TODO: https://tokio.rs/tokio/topics/bridging
    pub fn write_command(&mut self, command: String) {
        let addr = SocketAddr::from(([127, 0, 0, 1], 2000));
        let bytes = command.as_bytes();
        self.sock.send_to(bytes, addr).unwrap();
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
            let s = self.data.lock().unwrap();
            let data = s.get_data();
            serde_json::to_string(&data).unwrap()
        };

        std::str::from_utf8(stringified.as_bytes()).unwrap().into()
    }
}
