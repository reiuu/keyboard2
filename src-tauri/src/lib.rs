use windows::Win32::Media::Audio::{midiOutGetNumDevs, midiOutGetDevCapsW, MIDIOUTCAPSW};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use serde::Serialize;

#[derive(Serialize)]
struct MidiDevice {
    id: u32,
    d_name: String,
}

#[tauri::command]
fn get_midi_devices() -> Vec<MidiDevice> {
    unsafe {
        // COM initialisation required for Windows API
        // CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap(); // single-threaded apartment

        let mut devices: Vec<MidiDevice> = Vec::new();
        let num_devices: u32 = midiOutGetNumDevs();
        
        for d in 0..num_devices {
            let mut caps = MIDIOUTCAPSW::default(); // value here will be filled by below, like 'out' in c#
            if midiOutGetDevCapsW(d as usize, &mut caps, size_of::<MIDIOUTCAPSW>() as u32) == 0u32 {
                let device_name = caps.szPname; // MIDIOUTCAPSW is packed struct, meaning fields arent properly aligned in memory, 
                                                           // which could lead to undefined behaviour when dereferencing. 
                                                           // so we instead store this in a variable for stability.

                let len = device_name
                    .iter()
                    .position(|&c| c == 0) // win32 api uses null terminated strings, so search arr for 0 to find end
                    .unwrap_or(device_name.len());

                let name = String::from_utf16_lossy(&device_name[..len]); // borrow device_name instead of moving it
                
                println!("Found device: {}", &name);

                let device = MidiDevice {
                    id: d,
                    d_name: name 
                };

                devices.push(device);
            }
        }

        return devices;     
    }
} 



pub fn run() {
    tauri::Builder::default() // see std::process::Command - common Rust pattern
        //.plugin(tauri_plugin_opener::init()) // file opener plugin, not required
        .invoke_handler(tauri::generate_handler![get_midi_devices]) // register commands
        .run(tauri::generate_context!()) // generate_context!() from tauri.conf.json
        .expect("error while running tauri application");
}
