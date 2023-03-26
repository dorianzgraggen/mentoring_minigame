use std::net::SocketAddr;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use bevy::prelude::*;

use tokio::sync::oneshot::Receiver as OneshotReceiver;
use tokio::sync::oneshot::Sender as OneshotSender;

use super::game_scene_plugin::Player;
use deno_core::error::AnyError;
use deno_core::include_js_files;
use deno_core::op;
use deno_core::Extension;
use deno_core::OpState;
use std::net::UdpSocket;
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

use std::sync::mpsc;

use tokio::net::UdpSocket as TokioUdpSocket;

pub struct JsPlugin;

impl Plugin for JsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_js_system);
    }
}

fn udp_server() -> std::io::Result<()> {
    //let socket = UdpSocket::bind("0.0.0.0:2000")?; // for UDP4
    let socket = UdpSocket::bind("127.0.0.1:2000")?; // for UDP4/6
    let mut buf = [0; 2048];

    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)?;
        println!("{:?} bytes received from {:?}", number_of_bytes, src_addr);
        let buf = &mut buf[..number_of_bytes];
        let s = std::str::from_utf8(buf).unwrap();

        println!("-----------------> arrived: {}", s);
    }
}

fn spawn_js_system() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    std::thread::spawn(move || {
        js_system(tx);
    });

    std::thread::spawn(udp_server);

    let received = rx.recv().unwrap();
    println!("------------ Got: {}", received);
}

fn js_system(tx: Sender<String>) {
    let file_path = "./app.js";

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let (tx2, rx2): (OneshotSender<String>, OneshotReceiver<String>) = oneshot::channel();

    let val = String::from("hi");
    tx.send(val).unwrap();

    if let Err(error) = runtime.block_on(run_js(file_path, tx2)) {
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
async fn op_sleep(milliseconds: u64) -> Result<(), AnyError> {
    sleep(Duration::from_millis(milliseconds)).await;
    Ok(())
}

async fn run_js(file_path: &str, tx: OneshotSender<String>) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;
    let runjs_extension = Extension::builder("runjs")
        .esm(include_js_files!("js_plugin_runtime.js",))
        .ops(vec![
            op_fetch::decl(),
            op_get_str::decl(),
            op_command::decl(),
            op_sleep::decl(),
        ])
        .build();
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });

    let op_state = js_runtime.op_state();

    let sock = UdpSocket::bind("0.0.0.0:8080").unwrap();
    let remote_addr = "[::]:2000";

    op_state.borrow_mut().put(Commander {
        value: "Hallo i bims".to_string(),
        tx,
        sock,
    });

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

struct Commander {
    pub value: String,
    tx: OneshotSender<String>,
    pub sock: UdpSocket,
}

impl Commander {
    // TODO: https://tokio.rs/tokio/topics/bridging
    pub fn write_command(&mut self, command: String) {
        // let buf = [0; 1024];

        let addr = SocketAddr::from(([127, 0, 0, 1], 2000));

        println!("should send {}", command);

        let bytes = command.as_bytes();
        println!("as bytes: {:#?}", bytes);

        self.sock.send_to(bytes, &addr).unwrap();

        // match self.tx.send(command) {
        //     Ok(_) => (),
        //     Err(error) => println!("Error: Commander.write_command: {}", error),
        // }
        // match std::fs::write("./lol.txt", command) {
        //     Ok(_) => (),
        //     Err(error) => println!("fail {}", error),
        // };
    }
}
