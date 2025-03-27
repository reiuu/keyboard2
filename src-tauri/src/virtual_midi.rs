use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_ulonglong};
use std::ptr;
use widestring::WideCString;
use winapi::shared::minwindef::{BOOL, DWORD, LPBYTE, PDWORD, WORD};
use winapi::shared::ntdef::{LPCWSTR, LPWSTR};
use libloading::{Library, Symbol};

pub const TE_VM_DEFAULT_BUFFER_SIZE: DWORD = 0x1fffe;
pub const TE_VM_FLAGS_PARSE_RX: DWORD = 1;
pub const TE_VM_FLAGS_PARSE_TX: DWORD = 2;
pub const TE_VM_FLAGS_INSTANTIATE_RX_ONLY: DWORD = 4;
pub const TE_VM_FLAGS_INSTANTIATE_TX_ONLY: DWORD = 8;
pub const TE_VM_FLAGS_INSTANTIATE_BOTH: DWORD = 12;
pub const TE_VM_FLAGS_SUPPORTED: DWORD = TE_VM_FLAGS_PARSE_RX
    | TE_VM_FLAGS_PARSE_TX
    | TE_VM_FLAGS_INSTANTIATE_RX_ONLY
    | TE_VM_FLAGS_INSTANTIATE_TX_ONLY;

/*
 * Pointer to an opened teVirtualMIDI-port.  The data referenced by this pointer is for internal
 * use only, considered to be opaque and can change with each revision of the DLL.
 */
#[repr(C)]
pub struct VM_MIDI_PORT {
    _private: [u8; 0],
}
pub type LPVM_MIDI_PORT = *mut VM_MIDI_PORT;

/* Callback interface.  This callback is called by the driver/interface-dll for a packet of MIDI-data that is received from the driver
 * by the application using the virtual MIDI-port.
 *
 * This callback is called in an arbitrary thread-context - so make sure you have all your locking in order!
 *
 * If you have created the virtual-MIDI-port and specified TE_VM_FLAGS_PARSE_RX in the flags parameter, you will
 * receive a fully valid, preparsed MIDI-command with each callback.  The maximum size of data will be the amount
 * you specified in maxSysexLength.  Invalid commands or Sysex-commands with a length in excess of maxSysexLength
 * will be discarded and not forwarded to you.  Realtime-MIDI-commands will never be "intermingled" with other
 * commands (either normal or Sysex) in this mode.  If a realtime-MIDI-command is detected, it is sent to the
 * application before the command that it was intermingled with.
 *
 * In case of the driver being deactivated, the callback is called one time with a midiDataBytes==NULL and
 * length==zero, either the driver has been disabled, or another application using the driver has started
 * the installation of a newer driver-version
 *
 * You can throttle the speed of your virtualMIDI-port by not returning immediately from
 * this callback after you have taken care of the data received.
 *
 * If you want to throttle to 31250 bps for example, you need to place this line
 * before you return from your callback-function:
 *
 * Sleep( length * 10 * 1000) / 31250 );
 */
pub type LPVM_MIDI_DATA_CB = extern "system" fn(
    midiPort: LPVM_MIDI_PORT,
    midiDataBytes: LPBYTE,
    length: DWORD,
    dwCallbackInstance: usize,
);



pub struct VirtualMidi {
    lib: Library,
}

impl VirtualMidi {
    pub fn new() -> Result<Self, String> {
        let lib_names = ["teVirtualMIDI.dll", "teVirtualMIDI64.dll"];
        let mut last_error = None;

        for name in &lib_names {
            match unsafe { Library::new(name) } {
                Ok(lib) => return Ok(VirtualMidi { lib }),
                Err(e) => last_error = Some(e),
            }
        }

        Err(//VirtualMidiError::LibraryLoadError(format!(
            "Failed to load teVirtualMIDI DLL. Last error: {:?}".to_string()//,
            //last_error.unwrap()
        )//))
    }

    unsafe fn load_symbol<T>(&self, name: &str) -> Result<Symbol<T>, libloading::Error> {
        self.lib.get(name.as_bytes())
    }

    /* virtualMIDICreatePortEx2 - this is the current intended function to create a virtual MIDI-port.
    *
    * You can specify a name for the device to be created. Each named port can only exist once on a system.
    *
    * When the application terminates, the port will be deleted (or if the public front-end of the port is already in use by a DAW-application,
    * it will become inactive - giving back apropriate errors to the application using this port.
    *
    * In addition to the name, you can supply a callback-interface, which will be called for all MIDI-data received by the virtual-midi port.
    * You can also provide instance-data, which will also be handed back within the callback, to have the ability to reference port-specific
    * data-structures within your application.
    *
    * If you specify "NULL" for the callback function, you will not receive any callback, but can call the blocking function "virtualMIDIGetData"
    * to retrieve received MIDI-data/commands.  This is especially useful if one wants to interface this library to managed code like .NET or
    * Java, where callbacks into managed code are potentially complex or dangerous.  A call to virtualMIDIGetData when a callback has been
    * set during the creation will return with "ERROR_INVALID_FUNCTION".
    *
    * If you specified TE_VM_FLAGS_PARSE_RX in the flags parameter, you will always get one fully valid, preparsed MIDI-command in each callback.
    * In maxSysexLength you should specify a value that is large enough for the maximum size of Sysex that you expect to receive.  Sysex-commands
    * larger than the value specified here will be discarded and not sent to the user.  Realtime-MIDI-commands will never be "intermingled" with
    * other commands (either normal or Sysex) in this mode.  If a realtime-MIDI-command is detected, it is sent to the application before the
    * command that it was intermingled with.
    *
    * If you specify a maxSysexLength smaller than 2, you will receive fully valid preparsed MIDI-commands, but no Sysex-commands, since a
    * Sysex-command must be at least composed of 0xf0 + 0xf7 (start and end of sysex).  Since the parser will never be able to construct a
    * valid Sysex, you will receive none - but all other MIDI-commands will be parsed out and sent to you.
    *
    * When a NULL-pointer is handed back to the application, creation failed.  You can check GetLastError() to find out the specific problem
    * why the port could not be created.
    */
    pub unsafe fn create_port_ex2(
        &self,
        port_name: &str,
        callback: Option<LPVM_MIDI_DATA_CB>,
        callback_instance: usize,
        max_sysex_length: DWORD,
        flags: DWORD,
    ) -> Result<LPVM_MIDI_PORT, String> {
        let port_name = WideCString::from_str(port_name)
            .map_err(|e| format!("Invalid port name: {}", e))?;
    
        let func: Symbol<unsafe extern "system" fn(
            LPCWSTR,
            LPVM_MIDI_DATA_CB,
            usize,
            DWORD,
            DWORD,
        ) -> LPVM_MIDI_PORT> = self.load_symbol("virtualMIDICreatePortEx2")
            .map_err(|e| format!("Failed to load virtualMIDICreatePortEx2: {}", e))?;
    
        let cb = callback.unwrap_or_else(|| std::mem::transmute(ptr::null_mut::<c_void>()));
    
        let port = func(
            port_name.as_ptr(),
            cb,
            callback_instance,
            max_sysex_length,
            flags,
        );
    
        if port.is_null() {
            Err("Failed to create MIDI port (returned null)".to_string())
        } else {
            Ok(port)
        }
    }

    /* With this function, you can close a virtual MIDI-port again, after you have instanciated it.
    *
    * After the return from this function, no more callbacks will be received.
    *
    * Beware: do not call this function from within the midi-port-data-callback.  This may result in a deadlock!
    */
    pub unsafe fn close_port(&self, midi_port: LPVM_MIDI_PORT) -> Result<(), String> {
        let func: Symbol<unsafe extern "system" fn(LPVM_MIDI_PORT)> =
            self.load_symbol("virtualMIDIClosePort").expect("err");
        func(midi_port);
        Ok(())
    }

    /* With this function you can send a buffer of MIDI-data to the driver / the application that opened the virtual-MIDI-port.
    * If this function returns false, you may check GetLastError() to find out what caused the problem.
    *
    * This function should always be called with a single complete and valid midi-command (1-3 octets, or possibly more
    * for sysex).  Sysex-commands should not be split!  Realtime-MIDI-commands shall not be intermingled with other MIDI-
    * commands, but sent seperately!
    *
    * The data-size that can be used to send data to the virtual ports may be limited in size to prevent
    * an erratic application to allocate too much of the limited kernel-memory thus interfering with
    * system-stability.  The current limit is 512kb.
    */
    pub unsafe fn send_data(
        &self,
        midi_port: LPVM_MIDI_PORT,
        data: &[u8],
    ) -> Result<(), String> {
        let func: Symbol<unsafe extern "system" fn(LPVM_MIDI_PORT, LPBYTE, DWORD) -> BOOL> =
            self.load_symbol("virtualMIDISendData").expect("err");

        if func(midi_port, data.as_ptr() as LPBYTE, data.len() as DWORD) == 0 {
            Err("virtualMIDISendData failed".to_string())
        } else {
            Ok(())
        }
    }

    /* With this function you can use virtualMIDI without usage of callbacks.  This is especially interesting
    * if you want to interface the DLL to managed environments like Java or .NET where callbacks from native
    * to managed code are more complex.
    *
    * To use it, you need to open a virtualMIDI-port specifying NULL as callback.  If you have specified a
    * callback when opening the port, this function will fail - you cannot mix callbacks & reading via this
    * function.
    *
    * You need to provide a buffer large enough to retrieve the amount of data available.  Otherwise the
    * function will fail and return to you the necessary size in the length parameter.  If you specify
    * midiDataBytes to be NULL, the function will succeed but only return the size of buffer necessary
    * to retrieve the next MIDI-packet.
    *
    * virtualMIDIGetData will block until a complete block of data is available.  Depending on the fact if
    * you have specified to parse data into valid commands or just chunks of unprocessed data, you will
    * either receive the unparsed chunk (possibly containing multiple MIDI-commands), or a single, fully
    * valid MIDI-command.  In both cases, the length parameter will be filled with the length of data retrieved.
    *
    * You may only call virtualMIDIGetData once concurrently.  A call to this function will fail if another
    * call to this function is still not completed.
    */
    pub unsafe fn get_data(
        &self,
        midi_port: LPVM_MIDI_PORT,
        buffer: Option<&mut [u8]>,
    ) -> Result<DWORD, String> {
        let func: Symbol<unsafe extern "system" fn(LPVM_MIDI_PORT, LPBYTE, PDWORD) -> BOOL> =
            self.load_symbol("virtualMIDIGetData")
                .map_err(|e| format!("Failed to load symbol: {}", e))?;
    
        let mut length = if let Some(buf) = &buffer {
            buf.len() as DWORD
        } else {
            0
        };
    
        let buf_ptr = buffer
            .map(|b| b.as_mut_ptr() as LPBYTE)
            .unwrap_or(ptr::null_mut());
    
        if func(midi_port, buf_ptr, &mut length) == 0 {
            Err("virtualMIDIGetData function failed".to_string())
        } else {
            Ok(length)
        }
    }
}