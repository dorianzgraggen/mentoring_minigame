use std::rc::Rc;

use bevy::prelude::*;

use deno_core::error::AnyError;
use deno_core::include_js_files;
use deno_core::op;
use deno_core::Extension;
use deno_core::OpState;
use tokio::time::{sleep, Duration};

use super::game_scene_plugin::Player;

pub struct JsPlugin;

impl Plugin for JsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(js_system);
    }
}

fn js_system() {
    let file_path = "./app.js";

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_js(file_path)) {
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

async fn run_js(file_path: &str) -> Result<(), AnyError> {
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
    op_state.borrow_mut().put(Commander {
        value: "Hallo i bims".to_string(),
    });

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

struct Commander {
    pub value: String,
}

impl Commander {
    // TODO: https://tokio.rs/tokio/topics/bridging
    pub fn write_command(&self, command: String) {
        match std::fs::write("./lol.txt", command) {
            Ok(_) => (),
            Err(error) => println!("fail {}", error),
        };
    }
}
