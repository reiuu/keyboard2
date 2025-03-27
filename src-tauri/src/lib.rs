use windows::Win32::Media::Audio::{midiOutGetNumDevs, midiOutGetDevCapsW, MIDIOUTCAPSW};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED, COINIT_MULTITHREADED};
use serde::Serialize;

/* #[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
} */

#[derive(Serialize)]
struct MidiDevice {
    id: u32,
    name: String,
}

 #[tauri::command]
fn get_midi_devices() -> Vec<MidiDevice> {
    unsafe {
        let x = midiOutGetNumDevs();
        // COM initialisation required for Windows API
        CoInitializeEx(None, COINIT_APARTMENTTHREADED); // single-threaded apartment
        
        
    }
    return Vec::new();
} 



pub fn run() {
    tauri::Builder::default() // see std::process::Command - common Rust pattern
        //.plugin(tauri_plugin_opener::init()) // file opener plugin, not required
        .invoke_handler(tauri::generate_handler![get_midi_devices]) // register commands
        .run(tauri::generate_context!()) // generate_context!() from tauri.conf.json
        .expect("error while running tauri application");
}
