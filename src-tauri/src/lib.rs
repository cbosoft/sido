mod app_state;
mod init_snd;

use std::sync::Mutex;

use tauri::{Builder, Manager, generate_context, generate_handler, State};

use app_state::AppState;
use init_snd::init_snd;

#[tauri::command]
fn boop(state: State<'_, Mutex<AppState>>) {
  let mut app = state.lock().unwrap();
  app.boop();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (stream, net) = init_snd();
    Builder::default()
        .setup(move |app|{
            app.manage(Mutex::new(AppState::new(net)));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handler![boop])
        .run(generate_context!())
        .expect("error while running tauri application");
}
