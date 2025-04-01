use crate::Key;

// Row: 1 (Number Row)
pub const KEY_1: Key = Key(0x1E);
pub const KEY_2: Key = Key(0x1F);
pub const KEY_3: Key = Key(0x20);
pub const KEY_4: Key = Key(0x21);
pub const KEY_5: Key = Key(0x22);
pub const KEY_6: Key = Key(0x23);
pub const KEY_7: Key = Key(0x24);
pub const KEY_8: Key = Key(0x25);
pub const KEY_9: Key = Key(0x26);
pub const KEY_0: Key = Key(0x27);

pub const Q: Key = Key(0x14);

// Row: QWERTY (left to right)
pub const KEY_Q: Key = Key(0x14);
pub const KEY_W: Key = Key(0x1A);
pub const KEY_E: Key = Key(0x08);
pub const KEY_R: Key = Key(0x15);
pub const KEY_T: Key = Key(0x17);
pub const KEY_Y: Key = Key(0x1C);
pub const KEY_U: Key = Key(0x18);
pub const KEY_I: Key = Key(0x0C);
pub const KEY_O: Key = Key(0x12);
pub const KEY_P: Key = Key(0x13);

// Row: ASDF
pub const KEY_A: Key = Key(0x04);
pub const KEY_S: Key = Key(0x16);
pub const KEY_D: Key = Key(0x07);
pub const KEY_F: Key = Key(0x09);
pub const KEY_G: Key = Key(0x0A);
pub const KEY_H: Key = Key(0x0B);
pub const KEY_J: Key = Key(0x0D);
pub const KEY_K: Key = Key(0x0E);
pub const KEY_L: Key = Key(0x0F);

// Row: ZXCV
pub const KEY_Z: Key = Key(0x1D);
pub const KEY_X: Key = Key(0x1B);
pub const KEY_C: Key = Key(0x06);
pub const KEY_V: Key = Key(0x19);
pub const KEY_B: Key = Key(0x05);
pub const KEY_N: Key = Key(0x11);
pub const KEY_M: Key = Key(0x10);

// Spacebar + modifiers
pub const SPACE: Key = Key(0x2C);
pub const ENTER: Key = Key(0x28);
pub const ESC: Key = Key(0x29);
pub const BACKSPACE: Key = Key(0x2A);
pub const TAB: Key = Key(0x2B);
pub const CAPS_LOCK: Key = Key(0x39);

// Modifiers
pub const LSHIFT: Key = Key(0xE1);
pub const RSHIFT: Key = Key(0xE5);
pub const LCTRL: Key = Key(0xE0);
pub const RCTRL: Key = Key(0xE4);
pub const LALT: Key = Key(0xE2);
pub const RALT: Key = Key(0xE6);
pub const LGUI: Key = Key(0xE3); // Windows key / Cmd
pub const RGUI: Key = Key(0xE7);