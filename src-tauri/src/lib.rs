pub mod virtual_midi;

use virtual_midi::{VirtualMidi, TE_VM_FLAGS_INSTANTIATE_BOTH, TE_VM_FLAGS_PARSE_RX, VM_MIDI_PORT};
use windows::Win32::Media::Audio::{midiOutGetDevCapsW, midiOutGetNumDevs, midiOutOpen, HMIDIOUT, MIDIOUTCAPSW};
use serde::Serialize;


#[derive(Serialize)]
struct MidiDevice {
    id: u32,
    d_name: String,
}

#[tauri::command]
fn test_lib() {
    create_midi().expect("Error when creating MIDI port");
}

fn create_midi() -> Result<(), String> {

    let vm = VirtualMidi::new()?;
     
    let port = unsafe {
        vm.create_port_ex2(
            "My Rust MIDI Port",
            None,
            0, // callback instance data
            65535, // max sysex length
            TE_VM_FLAGS_PARSE_RX | TE_VM_FLAGS_INSTANTIATE_BOTH,
        ).expect("Test")
    };
    
    // send data (extract into seperate function so we can send using front-end)
    let midi_data = [0x90, 0x3C, 0x7F]; // note-on msg, C4
    unsafe { let _ = vm.send_data(port, &midi_data); } // let _ to hold possible err
    
    // leave open
    println!("Port is open, press Enter to close...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    unsafe { let _ = vm.close_port(port); }
    
    println!("Port is closed");
    Ok(())
}

/* #[tauri::command]
fn test_lib() {
    unsafe {
        let lib = Library::new("C:\\Users\\rynml\\Developer\\keyboard2\\src-tauri\\libs\\teVirtualMIDI64.dll").expect("failed to load DLL");
        let create_port: Symbol<unsafe extern "C" fn(&str, Option<unsafe extern "C" fn(...)>, u32, u32, *mut c_void) -> *mut c_void,> 
            = lib.get(b"virtualMIDI_CreatePortEx2").expect("failed to load function");
        let port = create_port("test1", 0, 1024, std::ptr::null_mut());

        println!("Created MIDI port: {:?}", port);
    }
} */

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
        .invoke_handler(tauri::generate_handler![get_midi_devices, test_lib]) // register commands
        .run(tauri::generate_context!()) // generate_context!() from tauri.conf.json
        .expect("error while running tauri application");
}
