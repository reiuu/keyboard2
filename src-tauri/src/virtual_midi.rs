#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};
use std::fmt;

pub struct VirtualMidiPort {
    ptr: LPVM_MIDI_PORT,
}

#[derive(Debug)]
pub struct VirtualMidiError(u32); // Windows error code

impl fmt::Display for VirtualMidiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "virtualMIDI error (win32 code {}): 0x{:X}", self.0, self.0)
    }
}

impl std::error::Error for VirtualMidiError {}

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

impl VirtualMidiPort {
    pub fn new_rx_only(name: &str) -> Result<Self, VirtualMidiError> {
        let wide_name = to_wide(name);
        let ptr = unsafe {
            virtualMIDICreatePortEx2(
                wide_name.as_ptr(),
                None,
                0,
                TE_VM_DEFAULT_BUFFER_SIZE,
                TE_VM_FLAGS_INSTANTIATE_RX_ONLY,
            )
        };

        if ptr.is_null() {
            let err = unsafe { winapi::um::errhandlingapi::GetLastError() };
            Err(VirtualMidiError(err))
        } else {
            Ok(VirtualMidiPort { ptr })
        }
    }

    pub fn send(&self, data: &[u8]) -> Result<(), VirtualMidiError> {
        let ok = unsafe {
            virtualMIDISendData(self.ptr, data.as_ptr() as *mut _, data.len() as u32)
        };
        if ok != 0 {
            Ok(())
        } else {
            let err = unsafe { winapi::um::errhandlingapi::GetLastError() };
            Err(VirtualMidiError(err))
        }
    }

    pub fn shutdown(&self) -> Result<(), VirtualMidiError> {
        let ok = unsafe { virtualMIDIShutdown(self.ptr) };
        if ok != 0 {
            Ok(())
        } else {
            let err = unsafe { winapi::um::errhandlingapi::GetLastError() };
            Err(VirtualMidiError(err))
        }
    }
}

impl Drop for VirtualMidiPort {
    fn drop(&mut self) {
        unsafe { virtualMIDIClosePort(self.ptr) };
    }
}
