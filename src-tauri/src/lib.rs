#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::{collections::HashMap, thread, time};
use windows::Win32::Media::Audio::{midiOutGetDevCapsW, midiOutGetNumDevs, MIDIOUTCAPSW};
use wooting_analog_wrapper::{self, initialise, read_full_buffer, set_keycode_mode, KeycodeType::HID};

pub mod definitions;
pub use definitions::keys::*;
pub use definitions::notes::*;
pub use definitions::types::*;

// For now, this is just creates a static KeyMap for testing, in future will programmatically generate
fn make_keymap() -> KeyMap {
    HashMap::from([
        (KEY_Q, C4),
        (KEY_W, D4),
        (KEY_E, E4),
        (KEY_R, F4),
        (KEY_T, G4),
        (KEY_Y, A4),
        (KEY_U, B4),
        (KEY_I, C5),
    ]).into()
}

fn create_midi_device() -> Result<(), String> {
    unsafe {
        let port_name = to_wide("keyboard2 MIDI");
        let port = virtualMIDICreatePortEx2(
            port_name.as_ptr(), 
            None, 
            0, 
            TE_VM_DEFAULT_BUFFER_SIZE, 
            TE_VM_FLAGS_PARSE_RX | TE_VM_FLAGS_INSTANTIATE_BOTH);

        if port.is_null() {
            return Err("Failed to create MIDI device.".to_string()); 
        }  
    }

    Ok(())
} 

fn wooting_init() -> () {
    initialise();
    set_keycode_mode(HID);
}

fn get_keyboard_input() -> () {
    thread::spawn(|| {
        loop {
            let read_result = read_full_buffer(8);
            println!("Result: {:?}", read_result);

            // Algorithm (naive): 
            // read_results contains HashMap of length 0-8, of {Key, AnalogValue}
            // For each Key, check if it exists in our KeyMap.
            // If it does, get the corresponding MIDI value. Then add to pressed_keys. Else, ignore.
            // If we reach a threshhold trigger, store {KeyCode: Vec<AnalogValue, Timestamp>} in pressed_keys
            // If pressed_keys is not empty, do the following every iteration of the loop for each element in pressed_keys:
            //      After first trigger has been reached, start measuring unit/ms average of last n AnalogValue until we reach second trigger.
            //      We do this by tracking n values and sum of those values.
            //      After second trigger has been reached, or AnalogValue starts decreasing, calculate velocity (running sum/n values, mapped by dt time -> 0-127)
            //      Then assign (Note, Velocity) to MIDI object, and discard KeyCode from pressed_keys

            thread::sleep(time::Duration::from_millis(1));
        }
    });
}

fn velocity_calc(/* AnalogValue, TimeStamp */) -> () {
    


}

#[tauri::command]
fn get_midi_devices() -> Vec<MidiDevice> {
    unsafe {
        // COM initialisation required for Windows API
        // CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap(); // single-threaded apartment

        let mut devices: Vec<MidiDevice> = Vec::new();
        let num_devices: u32 = midiOutGetNumDevs();
        
        for d in 1..num_devices {
            let mut caps = MIDIOUTCAPSW::default(); // value here will be filled by below, like 'out' in c#
            if midiOutGetDevCapsW(d as usize, &mut caps, size_of::<MIDIOUTCAPSW>() as u32) == 0u32 {
                let szPname = caps.szPname; // MIDIOUTCAPSW is packed struct, meaning fields arent properly aligned in memory, 
                                                           // which could lead to undefined behaviour when dereferencing. 
                                                           // so we instead store this in a variable for stability.

                let end_idx = szPname
                    .iter()
                    .position(|&c| c == 0) // win32 api uses null terminated strings, so search arr for 0 to find end
                    .unwrap_or(szPname.len());

                let device_name = String::from_utf16_lossy(&szPname[..end_idx]); // borrow device_name instead of moving it

                println!("Found device: {} ID: {}", &device_name, &d);

                let device = MidiDevice {
                    id: d,
                    name: device_name 
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
            create_midi_device().expect("MIDI device initialisation failed.");
            wooting_init();
            get_keyboard_input();
            Ok(())
        }) // see std::process::Command - common Rust pattern
        //.plugin(tauri_plugin_opener::init()) // file opener plugin, not required
        .invoke_handler(tauri::generate_handler![get_midi_devices]) // register commands
        .run(tauri::generate_context!()) // generate_context!() from tauri.conf.json
        .expect("error while running tauri application");
}
