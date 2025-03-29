#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::thread;
use windows::Win32::Media::Audio::{midiOutGetDevCapsW, midiOutGetNumDevs, MIDIOUTCAPSW};
use serde::Serialize;
mod ffi_helpers;

#[derive(Serialize)]
struct MidiDevice {
    id: u32,
    d_name: String,
}

fn create_midi() -> Result<(), String> {
    thread::spawn(|| {
        unsafe {
            let port_name = ffi_helpers::to_wide("keyboard2 MIDI");
            let _port = virtualMIDICreatePortEx2(
                port_name.as_ptr(), 
                None, 
                0, 
                TE_VM_DEFAULT_BUFFER_SIZE, 
                TE_VM_FLAGS_PARSE_RX | TE_VM_FLAGS_INSTANTIATE_BOTH);

            thread::park();
        }
    });

    Ok(())
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

                let end_idx = device_name
                    .iter()
                    .position(|&c| c == 0) // win32 api uses null terminated strings, so search arr for 0 to find end
                    .unwrap_or(device_name.len());

                let name = String::from_utf16_lossy(&device_name[..end_idx]); // borrow device_name instead of moving it
                
                println!("Found device: {} ID: {}", &name, &d);

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
    tauri::Builder::default()
        .setup(|_app| {
            create_midi().expect("MIDI thread failed");
            Ok(())
        }) // see std::process::Command - common Rust pattern
        //.plugin(tauri_plugin_opener::init()) // file opener plugin, not required
        .invoke_handler(tauri::generate_handler![get_midi_devices]) // register commands
        .run(tauri::generate_context!()) // generate_context!() from tauri.conf.json
        .expect("error while running tauri application");
}
