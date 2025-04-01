use std::{collections::HashMap, ffi::OsStr};
use std::os::windows::ffi::OsStrExt;
use serde::Serialize;

#[derive(Serialize)]
pub struct MidiDevice {
    pub id: u32,
    pub name: String,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnalogValue(pub f32);

#[derive(Debug, Clone, PartialEq)]
pub struct AnalogKey(pub Key, pub AnalogValue); 

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Note(pub u8);

#[derive(Debug, Clone, PartialEq)]
pub struct KeyMap(pub HashMap<Key, Note>);

impl From<HashMap<Key, Note>> for KeyMap {
    fn from(map: HashMap<Key, Note>) -> Self {
        KeyMap(map)
    }
}

// Converts a Rust string to OS preferred wide-string.
pub fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect() // chain to append a null terminator
}

