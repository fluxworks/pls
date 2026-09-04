//! Windows platform support

pub use self::console::terminal_read;

mod console;
pub mod path;

#[macro_export] macro_rules! UNION {
    ($(#[$attrs:meta])* union $name:ident {
        [$stype:ty; $ssize:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])*
        pub struct $name([$stype; $ssize]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
    ($(#[$attrs:meta])* union $name:ident {
        [$stype32:ty; $ssize32:expr] [$stype64:ty; $ssize64:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])* #[cfg(target_pointer_width = "32")]
        pub struct $name([$stype32; $ssize32]);
        #[repr(C)] $(#[$attrs])* #[cfg(target_pointer_width = "64")]
        pub struct $name([$stype64; $ssize64]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
}


#[macro_export] macro_rules! STRUCT {
    (#[debug] $($rest:tt)*) => (
        STRUCT!{#[cfg_attr(feature = "impl-debug", derive(Debug))] $($rest)*}
    );
    ($(#[$attrs:meta])* struct $name:ident {
        $($field:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] #[derive(Copy)] $(#[$attrs])*
        pub struct $name {
            $(pub $field: $ftype,)+
        }
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
    );
}

pub mod ctypes
{
    pub enum c_void {}
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;
    pub type __int8 = i8;
    pub type __uint8 = u8;
    pub type __int16 = i16;
    pub type __uint16 = u16;
    pub type __int32 = i32;
    pub type __uint32 = u32;
    pub type __int64 = i64;
    pub type __uint64 = u64;
    pub type wchar_t = u16;

    pub type DWORD = c_ulong;
    pub type BOOL = c_int;
    pub type BYTE = c_uchar;
    pub type WORD = c_ushort;
    pub type FLOAT = c_float;

    pub type VOID = c_void;
    pub type CHAR = c_char;
    pub type SHORT = c_short;
    pub type LONG = c_long;
    pub type INT = c_int;
    pub type UINT = c_uint;
    pub type PUINT = *mut c_uint;

    pub type WCHAR = wchar_t;
}

pub mod shared
{
    use crate::
    {
        *,
    };

    pub mod minwindef
    {
        use crate::
        {
            *,
        };

        use super::super::ctypes::{ * };
        pub const FALSE: BOOL = 0;
        pub const TRUE: BOOL = 1;

        pub type DWORD = c_ulong;
        pub type BOOL = c_int;
        pub type BYTE = c_uchar;
        pub type WORD = c_ushort;
        pub type FLOAT = c_float;
    }
}

pub mod um
{
    use crate::
    {
        *,
    };

    pub mod wincon
    {
        use crate::
        {
            *,
        };

        use super::super::ctypes::{ * };

        pub const KEY_EVENT: WORD = 0x0001;
        pub const MOUSE_EVENT: WORD = 0x0002;
        pub const WINDOW_BUFFER_SIZE_EVENT: WORD = 0x0004;
        pub const MENU_EVENT: WORD = 0x0008;
        pub const FOCUS_EVENT: WORD = 0x0010;

        pub const RIGHT_ALT_PRESSED: DWORD = 0x0001;
        pub const LEFT_ALT_PRESSED: DWORD = 0x0002;
        pub const RIGHT_CTRL_PRESSED: DWORD = 0x0004;
        pub const LEFT_CTRL_PRESSED: DWORD = 0x0008;
        pub const SHIFT_PRESSED: DWORD = 0x0010;
        pub const NUMLOCK_ON: DWORD = 0x0020;
        pub const SCROLLLOCK_ON: DWORD = 0x0040;
        pub const CAPSLOCK_ON: DWORD = 0x0080;
        pub const ENHANCED_KEY: DWORD = 0x0100;
        pub const NLS_DBCSCHAR: DWORD = 0x00010000;
        pub const NLS_ALPHANUMERIC: DWORD = 0x00000000;
        pub const NLS_KATAKANA: DWORD = 0x00020000;
        pub const NLS_HIRAGANA: DWORD = 0x00040000;
        pub const NLS_ROMAN: DWORD = 0x00400000;
        pub const NLS_IME_CONVERSION: DWORD = 0x00800000;
        pub const NLS_IME_DISABLE: DWORD = 0x20000000;

        STRUCT!{struct FOCUS_EVENT_RECORD {
            bSetFocus: BOOL,
        }}
        pub type PFOCUS_EVENT_RECORD = *mut FOCUS_EVENT_RECORD;

        STRUCT!{struct MENU_EVENT_RECORD {
            dwCommandId: UINT,
        }}

        STRUCT!{struct COORD {
            X: SHORT,
            Y: SHORT,
        }}
        pub type PCOORD = *mut COORD;

        STRUCT!{struct WINDOW_BUFFER_SIZE_RECORD {
            dwSize: COORD,
        }}
        pub type PWINDOW_BUFFER_SIZE_RECORD = *mut WINDOW_BUFFER_SIZE_RECORD;

        STRUCT!{struct MOUSE_EVENT_RECORD {
            dwMousePosition: COORD,
            dwButtonState: DWORD,
            dwControlKeyState: DWORD,
            dwEventFlags: DWORD,
        }}
        pub type PMOUSE_EVENT_RECORD = *mut MOUSE_EVENT_RECORD;

        UNION!{union KEY_EVENT_RECORD_uChar {
            [u16; 1],
            UnicodeChar UnicodeChar_mut: WCHAR,
            AsciiChar AsciiChar_mut: CHAR,
        }}

        STRUCT!{struct KEY_EVENT_RECORD {
            bKeyDown: BOOL,
            wRepeatCount: WORD,
            wVirtualKeyCode: WORD,
            wVirtualScanCode: WORD,
            uChar: KEY_EVENT_RECORD_uChar,
            dwControlKeyState: DWORD,
        }}
        pub type PKEY_EVENT_RECORD = *mut KEY_EVENT_RECORD;

        UNION!{union INPUT_RECORD_Event {
            [u32; 4],
            KeyEvent KeyEvent_mut: KEY_EVENT_RECORD,
            MouseEvent MouseEvent_mut: MOUSE_EVENT_RECORD,
            WindowBufferSizeEvent WindowBufferSizeEvent_mut: WINDOW_BUFFER_SIZE_RECORD,
            MenuEvent MenuEvent_mut: MENU_EVENT_RECORD,
            FocusEvent FocusEvent_mut: FOCUS_EVENT_RECORD,
        }}
        STRUCT!{struct INPUT_RECORD {
            EventType: WORD,
            Event: INPUT_RECORD_Event,
        }}
        pub type PINPUT_RECORD = *mut INPUT_RECORD;
    }

    pub mod winuser
    {
        use crate::
        {
            *,
        };

        use super::super::ctypes::{ * };

        pub const VK_LBUTTON: c_int = 0x01;
        pub const VK_RBUTTON: c_int = 0x02;
        pub const VK_CANCEL: c_int = 0x03;
        pub const VK_MBUTTON: c_int = 0x04;
        pub const VK_XBUTTON1: c_int = 0x05;
        pub const VK_XBUTTON2: c_int = 0x06;
        pub const VK_BACK: c_int = 0x08;
        pub const VK_TAB: c_int = 0x09;
        pub const VK_CLEAR: c_int = 0x0C;
        pub const VK_RETURN: c_int = 0x0D;
        pub const VK_SHIFT: c_int = 0x10;
        pub const VK_CONTROL: c_int = 0x11;
        pub const VK_MENU: c_int = 0x12;
        pub const VK_PAUSE: c_int = 0x13;
        pub const VK_CAPITAL: c_int = 0x14;
        pub const VK_KANA: c_int = 0x15;
        pub const VK_HANGEUL: c_int = 0x15;
        pub const VK_HANGUL: c_int = 0x15;
        pub const VK_JUNJA: c_int = 0x17;
        pub const VK_FINAL: c_int = 0x18;
        pub const VK_HANJA: c_int = 0x19;
        pub const VK_KANJI: c_int = 0x19;
        pub const VK_ESCAPE: c_int = 0x1B;
        pub const VK_CONVERT: c_int = 0x1C;
        pub const VK_NONCONVERT: c_int = 0x1D;
        pub const VK_ACCEPT: c_int = 0x1E;
        pub const VK_MODECHANGE: c_int = 0x1F;
        pub const VK_SPACE: c_int = 0x20;
        pub const VK_PRIOR: c_int = 0x21;
        pub const VK_NEXT: c_int = 0x22;
        pub const VK_END: c_int = 0x23;
        pub const VK_HOME: c_int = 0x24;
        pub const VK_LEFT: c_int = 0x25;
        pub const VK_UP: c_int = 0x26;
        pub const VK_RIGHT: c_int = 0x27;
        pub const VK_DOWN: c_int = 0x28;
        pub const VK_SELECT: c_int = 0x29;
        pub const VK_PRINT: c_int = 0x2A;
        pub const VK_EXECUTE: c_int = 0x2B;
        pub const VK_SNAPSHOT: c_int = 0x2C;
        pub const VK_INSERT: c_int = 0x2D;
        pub const VK_DELETE: c_int = 0x2E;
        pub const VK_HELP: c_int = 0x2F;
        pub const VK_LWIN: c_int = 0x5B;
        pub const VK_RWIN: c_int = 0x5C;
        pub const VK_APPS: c_int = 0x5D;
        pub const VK_SLEEP: c_int = 0x5F;
        pub const VK_NUMPAD0: c_int = 0x60;
        pub const VK_NUMPAD1: c_int = 0x61;
        pub const VK_NUMPAD2: c_int = 0x62;
        pub const VK_NUMPAD3: c_int = 0x63;
        pub const VK_NUMPAD4: c_int = 0x64;
        pub const VK_NUMPAD5: c_int = 0x65;
        pub const VK_NUMPAD6: c_int = 0x66;
        pub const VK_NUMPAD7: c_int = 0x67;
        pub const VK_NUMPAD8: c_int = 0x68;
        pub const VK_NUMPAD9: c_int = 0x69;
        pub const VK_MULTIPLY: c_int = 0x6A;
        pub const VK_ADD: c_int = 0x6B;
        pub const VK_SEPARATOR: c_int = 0x6C;
        pub const VK_SUBTRACT: c_int = 0x6D;
        pub const VK_DECIMAL: c_int = 0x6E;
        pub const VK_DIVIDE: c_int = 0x6F;
        pub const VK_F1: c_int = 0x70;
        pub const VK_F2: c_int = 0x71;
        pub const VK_F3: c_int = 0x72;
        pub const VK_F4: c_int = 0x73;
        pub const VK_F5: c_int = 0x74;
        pub const VK_F6: c_int = 0x75;
        pub const VK_F7: c_int = 0x76;
        pub const VK_F8: c_int = 0x77;
        pub const VK_F9: c_int = 0x78;
        pub const VK_F10: c_int = 0x79;
        pub const VK_F11: c_int = 0x7A;
        pub const VK_F12: c_int = 0x7B;
        pub const VK_F13: c_int = 0x7C;
        pub const VK_F14: c_int = 0x7D;
        pub const VK_F15: c_int = 0x7E;
        pub const VK_F16: c_int = 0x7F;
        pub const VK_F17: c_int = 0x80;
        pub const VK_F18: c_int = 0x81;
        pub const VK_F19: c_int = 0x82;
        pub const VK_F20: c_int = 0x83;
        pub const VK_F21: c_int = 0x84;
        pub const VK_F22: c_int = 0x85;
        pub const VK_F23: c_int = 0x86;
        pub const VK_F24: c_int = 0x87;
        pub const VK_NAVIGATION_VIEW: c_int = 0x88;
        pub const VK_NAVIGATION_MENU: c_int = 0x89;
        pub const VK_NAVIGATION_UP: c_int = 0x8A;
        pub const VK_NAVIGATION_DOWN: c_int = 0x8B;
        pub const VK_NAVIGATION_LEFT: c_int = 0x8C;
        pub const VK_NAVIGATION_RIGHT: c_int = 0x8D;
        pub const VK_NAVIGATION_ACCEPT: c_int = 0x8E;
        pub const VK_NAVIGATION_CANCEL: c_int = 0x8F;
        pub const VK_NUMLOCK: c_int = 0x90;
        pub const VK_SCROLL: c_int = 0x91;
        pub const VK_OEM_NEC_EQUAL: c_int = 0x92;
        pub const VK_OEM_FJ_JISHO: c_int = 0x92;
        pub const VK_OEM_FJ_MASSHOU: c_int = 0x93;
        pub const VK_OEM_FJ_TOUROKU: c_int = 0x94;
        pub const VK_OEM_FJ_LOYA: c_int = 0x95;
        pub const VK_OEM_FJ_ROYA: c_int = 0x96;
        pub const VK_LSHIFT: c_int = 0xA0;
        pub const VK_RSHIFT: c_int = 0xA1;
        pub const VK_LCONTROL: c_int = 0xA2;
        pub const VK_RCONTROL: c_int = 0xA3;
        pub const VK_LMENU: c_int = 0xA4;
        pub const VK_RMENU: c_int = 0xA5;
        pub const VK_BROWSER_BACK: c_int = 0xA6;
        pub const VK_BROWSER_FORWARD: c_int = 0xA7;
        pub const VK_BROWSER_REFRESH: c_int = 0xA8;
        pub const VK_BROWSER_STOP: c_int = 0xA9;
        pub const VK_BROWSER_SEARCH: c_int = 0xAA;
        pub const VK_BROWSER_FAVORITES: c_int = 0xAB;
        pub const VK_BROWSER_HOME: c_int = 0xAC;
        pub const VK_VOLUME_MUTE: c_int = 0xAD;
        pub const VK_VOLUME_DOWN: c_int = 0xAE;
        pub const VK_VOLUME_UP: c_int = 0xAF;
        pub const VK_MEDIA_NEXT_TRACK: c_int = 0xB0;
        pub const VK_MEDIA_PREV_TRACK: c_int = 0xB1;
        pub const VK_MEDIA_STOP: c_int = 0xB2;
        pub const VK_MEDIA_PLAY_PAUSE: c_int = 0xB3;
        pub const VK_LAUNCH_MAIL: c_int = 0xB4;
        pub const VK_LAUNCH_MEDIA_SELECT: c_int = 0xB5;
        pub const VK_LAUNCH_APP1: c_int = 0xB6;
        pub const VK_LAUNCH_APP2: c_int = 0xB7;
        pub const VK_OEM_1: c_int = 0xBA;
        pub const VK_OEM_PLUS: c_int = 0xBB;
        pub const VK_OEM_COMMA: c_int = 0xBC;
        pub const VK_OEM_MINUS: c_int = 0xBD;
        pub const VK_OEM_PERIOD: c_int = 0xBE;
        pub const VK_OEM_2: c_int = 0xBF;
        pub const VK_OEM_3: c_int = 0xC0;
        pub const VK_GAMEPAD_A: c_int = 0xC3;
        pub const VK_GAMEPAD_B: c_int = 0xC4;
        pub const VK_GAMEPAD_X: c_int = 0xC5;
        pub const VK_GAMEPAD_Y: c_int = 0xC6;
        pub const VK_GAMEPAD_RIGHT_SHOULDER: c_int = 0xC7;
        pub const VK_GAMEPAD_LEFT_SHOULDER: c_int = 0xC8;
        pub const VK_GAMEPAD_LEFT_TRIGGER: c_int = 0xC9;
        pub const VK_GAMEPAD_RIGHT_TRIGGER: c_int = 0xCA;
        pub const VK_GAMEPAD_DPAD_UP: c_int = 0xCB;
        pub const VK_GAMEPAD_DPAD_DOWN: c_int = 0xCC;
        pub const VK_GAMEPAD_DPAD_LEFT: c_int = 0xCD;
        pub const VK_GAMEPAD_DPAD_RIGHT: c_int = 0xCE;
        pub const VK_GAMEPAD_MENU: c_int = 0xCF;
        pub const VK_GAMEPAD_VIEW: c_int = 0xD0;
        pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: c_int = 0xD1;
        pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: c_int = 0xD2;
        pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: c_int = 0xD3;
        pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: c_int = 0xD4;
        pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: c_int = 0xD5;
        pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: c_int = 0xD6;
        pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: c_int = 0xD7;
        pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: c_int = 0xD8;
        pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: c_int = 0xD9;
        pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: c_int = 0xDA;
        pub const VK_OEM_4: c_int = 0xDB;
        pub const VK_OEM_5: c_int = 0xDC;
        pub const VK_OEM_6: c_int = 0xDD;
        pub const VK_OEM_7: c_int = 0xDE;
        pub const VK_OEM_8: c_int = 0xDF;
        pub const VK_OEM_AX: c_int = 0xE1;
        pub const VK_OEM_102: c_int = 0xE2;
        pub const VK_ICO_HELP: c_int = 0xE3;
        pub const VK_ICO_00: c_int = 0xE4;
        pub const VK_PROCESSKEY: c_int = 0xE5;
        pub const VK_ICO_CLEAR: c_int = 0xE6;
        pub const VK_PACKET: c_int = 0xE7;
        pub const VK_OEM_RESET: c_int = 0xE9;
        pub const VK_OEM_JUMP: c_int = 0xEA;
        pub const VK_OEM_PA1: c_int = 0xEB;
        pub const VK_OEM_PA2: c_int = 0xEC;
        pub const VK_OEM_PA3: c_int = 0xED;
        pub const VK_OEM_WSCTRL: c_int = 0xEE;
        pub const VK_OEM_CUSEL: c_int = 0xEF;
        pub const VK_OEM_ATTN: c_int = 0xF0;
        pub const VK_OEM_FINISH: c_int = 0xF1;
        pub const VK_OEM_COPY: c_int = 0xF2;
        pub const VK_OEM_AUTO: c_int = 0xF3;
        pub const VK_OEM_ENLW: c_int = 0xF4;
        pub const VK_OEM_BACKTAB: c_int = 0xF5;
        pub const VK_ATTN: c_int = 0xF6;
        pub const VK_CRSEL: c_int = 0xF7;
        pub const VK_EXSEL: c_int = 0xF8;
        pub const VK_EREOF: c_int = 0xF9;
        pub const VK_PLAY: c_int = 0xFA;
        pub const VK_ZOOM: c_int = 0xFB;
        pub const VK_NONAME: c_int = 0xFC;
        pub const VK_PA1: c_int = 0xFD;
        pub const VK_OEM_CLEAR: c_int = 0xFE;
    }
}