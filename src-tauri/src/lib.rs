mod app_state;
mod init_snd;
mod marker;
mod note;
mod patch;

use std::sync::Mutex;

use tauri::{generate_context, generate_handler, ipc::Channel, Builder, Manager, State};

use app_state::AppState;
use init_snd::init_snd;
use note::SpecifiedNote;


#[tauri::command]
fn boop(state: State<'_, Mutex<AppState>>) {
    let mut app = state.lock().unwrap();
    app.boop();
}


#[tauri::command]
fn play_current_patch(state: State<'_, Mutex<AppState>>, note: SpecifiedNote) {
    let mut app = state.lock().unwrap();
    app.play_current_patch(note);
}


#[tauri::command]
fn play_sequence(state: State<'_, Mutex<AppState>>, bpm: f64, beats: u64, divisions: u64, notes: Vec<SpecifiedNote>, time_ind: Channel<u64>) {
    // let note: SpecifiedNote = serde_json::from_str(&note).unwrap();
    let mut app = state.lock().unwrap();
    app.play_sequence(bpm, beats, divisions, notes, time_ind);
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
        .invoke_handler(generate_handler![
            boop,
            play_current_patch,
            play_sequence
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
    let _ = stream;
}
