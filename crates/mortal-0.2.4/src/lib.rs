//! Platform-independent terminal interface

#![allow
(
    mismatched_lifetime_syntaxes,
    non_camel_case_types,
    non_snake_case,
    unused_imports,
)]

#[macro_use] extern crate bitflags;
extern crate smallstr;
extern crate unicode_normalization;
extern crate unicode_width;

#[cfg(unix)] extern crate libc;
#[cfg(unix)] extern crate nix;
#[cfg(unix)] extern crate terminfo;

//#[cfg(windows)] extern crate winapi;

pub use crate::screen::{Screen, ScreenReadGuard, ScreenWriteGuard};
pub use crate::sequence::{FindResult, SequenceMap};
pub use crate::signal::{Signal, SignalSet};
pub use crate::terminal::{
    Color, Cursor, CursorMode, Size, Style, Theme,
    Event, Key, MouseEvent, MouseInput, MouseButton, ModifierState,
    PrepareConfig, PrepareState,
    Terminal, TerminalReadGuard, TerminalWriteGuard,
};

#[macro_use] mod buffer;
#[doc(hidden)]
#[macro_use] pub mod macros;
mod priv_util;
pub mod screen;
pub mod sequence;
pub mod signal;
pub mod terminal;
pub mod util;

#[macro_export] macro_rules! FN
{
    (stdcall $func:ident($($t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "system" fn($($t,)*) -> $ret>;
    );
    (stdcall $func:ident($($p:ident: $t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "system" fn($($p: $t,)*) -> $ret>;
    );
    (cdecl $func:ident($($t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "C" fn($($t,)*) -> $ret>;
    );
    (cdecl $func:ident($($p:ident: $t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "C" fn($($p: $t,)*) -> $ret>;
    );
}

#[macro_export] macro_rules! STRUCT
{
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
    );
}

#[macro_export] macro_rules! UNION
{
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

pub mod ctypes
{
    pub use std::os::raw::c_void;
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

    pub type ULONG = c_ulong;
    pub type PULONG = *mut ULONG;
    pub type USHORT = c_ushort;
    pub type PUSHORT = *mut USHORT;
    pub type UCHAR = c_uchar;
    pub type PUCHAR = *mut UCHAR;
    pub type PSZ = *mut c_char;
    pub const MAX_PATH: usize = 260;
    pub const FALSE: BOOL = 0;
    pub const TRUE: BOOL = 1;
    pub type DWORD = c_ulong;
    pub type BOOL = c_int;
    pub type BYTE = c_uchar;
    pub type WORD = c_ushort;
    pub type FLOAT = c_float;
    pub type PFLOAT = *mut FLOAT;
    pub type PBOOL = *mut BOOL;
    pub type LPBOOL = *mut BOOL;
    pub type PBYTE = *mut BYTE;
    pub type LPBYTE = *mut BYTE;
    pub type PINT = *mut c_int;
    pub type LPINT = *mut c_int;
    pub type PWORD = *mut WORD;
    pub type LPWORD = *mut WORD;
    pub type LPLONG = *mut c_long;
    pub type PDWORD = *mut DWORD;
    pub type LPDWORD = *mut DWORD;
    pub type LPVOID = *mut c_void;
    pub type LPCVOID = *const c_void;
    pub type INT = c_int;
    pub type UINT = c_uint;
    pub type PUINT = *mut c_uint;
}

pub mod um
{
    use crate::
    {
        ctypes::{ * },
        *
    };

    pub mod consoleapi
    {
        use crate::
        {
            ctypes::{ * },
            shared::ntdef::{*},
            um::wincon::{ PCONSOLE_READCONSOLE_CONTROL, PINPUT_RECORD, PHANDLER_ROUTINE },
            *
        };

        #[link(name = "kernel32")]
        unsafe extern "system"
        {
            pub fn GetConsoleMode( hConsoleHandle:HANDLE, lpMode:LPDWORD ) -> BOOL;
            pub fn ReadConsoleW( hConsoleInput: HANDLE, lpBuffer: LPVOID, nNumberOfCharsToRead: DWORD, lpNumberOfCharsRead: LPDWORD, pInputControl: PCONSOLE_READCONSOLE_CONTROL ) -> BOOL;
            pub fn ReadConsoleInputW( hConsoleInput: HANDLE, lpBuffer: PINPUT_RECORD, nLength: DWORD, lpNumberOfEventsRead: LPDWORD ) -> BOOL;
            pub fn SetConsoleCtrlHandler( HandlerRoutine:PHANDLER_ROUTINE, Add:BOOL ) -> BOOL;
            pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL;
            pub fn WriteConsoleW( hConsoleOutput: HANDLE, lpBuffer: *const VOID, nNumberOfCharsToWrite: DWORD, lpNumberOfCharsWritten: LPDWORD, lpReserved: LPVOID ) -> BOOL;
        }
    }

    pub mod handleapi
    {
        use crate::
        {
            ctypes::{ * },
            shared::{ ntdef::{ HANDLE } },
            *
        };

        #[link(name = "kernel32")]
        unsafe extern "system"
        {
            pub fn CloseHandle( hObject: HANDLE ) -> BOOL;
        }
    }

    pub mod processenv
    {
        use crate::
        {
            ctypes::{ * },
            shared::{ ntdef::{ HANDLE } },
            *
        };

        #[link(name = "kernel32")]
        unsafe extern "system"
        {
            pub fn GetStdHandle( nStdHandle: DWORD ) -> HANDLE;
        }


    }

    pub mod synchapi
    {
        use crate::
        {
            ctypes::{ * },
            shared::{ ntdef::{ HANDLE } },
            *
        };

        #[link(name = "kernel32")]
        unsafe extern "system"
        {
            pub fn WaitForSingleObject( hHandle: HANDLE, dwMilliseconds: DWORD ) -> DWORD;
        }
    }

    pub mod winbase
    {
        use crate::
        {
            ctypes::{ * },
            um::
            {
                winnt::
                {
                    STATUS_WAIT_0, STATUS_ABANDONED_WAIT_0, STATUS_USER_APC
                },
            },
            *
        };

        pub const INFINITE: DWORD = 0xFFFFFFFF;

        pub const STD_INPUT_HANDLE: DWORD = -10i32 as u32;
        pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as u32;
        pub const STD_ERROR_HANDLE: DWORD = -12i32 as u32;

        pub const WAIT_FAILED: DWORD = 0xFFFFFFFF;
        pub const WAIT_OBJECT_0: DWORD = STATUS_WAIT_0 as u32;
        pub const WAIT_ABANDONED: DWORD = STATUS_ABANDONED_WAIT_0 as u32;
        pub const WAIT_ABANDONED_0: DWORD = STATUS_ABANDONED_WAIT_0 as u32;
        pub const WAIT_IO_COMPLETION: DWORD = STATUS_USER_APC as u32;
    }

    pub mod wincon
    {
        use crate::
        {
            ctypes::{ * },
            shared::
            {
                minwindef::{BOOL, DWORD, WORD},
                ntdef::{ * },
            },
            *
        };

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

        pub const FROM_LEFT_1ST_BUTTON_PRESSED: DWORD = 0x0001;
        pub const RIGHTMOST_BUTTON_PRESSED: DWORD = 0x0002;
        pub const FROM_LEFT_2ND_BUTTON_PRESSED: DWORD = 0x0004;
        pub const FROM_LEFT_3RD_BUTTON_PRESSED: DWORD = 0x0008;
        pub const FROM_LEFT_4TH_BUTTON_PRESSED: DWORD = 0x0010;
        pub const MOUSE_MOVED: DWORD = 0x0001;
        pub const DOUBLE_CLICK: DWORD = 0x0002;
        pub const MOUSE_WHEELED: DWORD = 0x0004;
        pub const MOUSE_HWHEELED: DWORD = 0x0008;

        pub const FOREGROUND_BLUE: WORD = 0x0001;
        pub const FOREGROUND_GREEN: WORD = 0x0002;
        pub const FOREGROUND_RED: WORD = 0x0004;
        pub const FOREGROUND_INTENSITY: WORD = 0x0008;
        pub const BACKGROUND_BLUE: WORD = 0x0010;
        pub const BACKGROUND_GREEN: WORD = 0x0020;
        pub const BACKGROUND_RED: WORD = 0x0040;
        pub const BACKGROUND_INTENSITY: WORD = 0x0080;
        pub const COMMON_LVB_LEADING_BYTE: WORD = 0x0100;
        pub const COMMON_LVB_TRAILING_BYTE: WORD = 0x0200;
        pub const COMMON_LVB_GRID_HORIZONTAL: WORD = 0x0400;
        pub const COMMON_LVB_GRID_LVERTICAL: WORD = 0x0800;
        pub const COMMON_LVB_GRID_RVERTICAL: WORD = 0x1000;
        pub const COMMON_LVB_REVERSE_VIDEO: WORD = 0x4000;
        pub const COMMON_LVB_UNDERSCORE: WORD = 0x8000;
        pub const COMMON_LVB_SBCSDBCS: WORD = 0x0300;

        FN!{stdcall PHANDLER_ROUTINE(
            CtrlType: DWORD,
        ) -> BOOL}

        STRUCT!{struct MENU_EVENT_RECORD {
            dwCommandId: UINT,
        }}

        STRUCT!{struct FOCUS_EVENT_RECORD {
            bSetFocus: BOOL,
        }}

        STRUCT!{struct WINDOW_BUFFER_SIZE_RECORD {
            dwSize: COORD,
        }}

        STRUCT!{struct MOUSE_EVENT_RECORD {
            dwMousePosition: COORD,
            dwButtonState: DWORD,
            dwControlKeyState: DWORD,
            dwEventFlags: DWORD,
        }}

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

        STRUCT!{struct CONSOLE_READCONSOLE_CONTROL {
            nLength: ULONG,
            nInitialChars: ULONG,
            dwCtrlWakeupMask: ULONG,
            dwControlKeyState: ULONG,
        }}

        pub type PCONSOLE_READCONSOLE_CONTROL = *mut CONSOLE_READCONSOLE_CONTROL;

        pub type PINPUT_RECORD = *mut INPUT_RECORD;
        pub const KEY_EVENT: WORD = 0x0001;
        pub const MOUSE_EVENT: WORD = 0x0002;
        pub const WINDOW_BUFFER_SIZE_EVENT: WORD = 0x0004;
        pub const MENU_EVENT: WORD = 0x0008;
        pub const FOCUS_EVENT: WORD = 0x0010;

        pub const CONSOLE_TEXTMODE_BUFFER: DWORD = 1;

        pub const CTRL_C_EVENT: DWORD = 0;
        pub const CTRL_BREAK_EVENT: DWORD = 1;
        pub const CTRL_CLOSE_EVENT: DWORD = 2;
        pub const CTRL_LOGOFF_EVENT: DWORD = 5;
        pub const CTRL_SHUTDOWN_EVENT: DWORD = 6;

        pub const ENABLE_PROCESSED_INPUT: DWORD = 0x0001;
        pub const ENABLE_LINE_INPUT: DWORD = 0x0002;
        pub const ENABLE_ECHO_INPUT: DWORD = 0x0004;
        pub const ENABLE_WINDOW_INPUT: DWORD = 0x0008;
        pub const ENABLE_MOUSE_INPUT: DWORD = 0x0010;
        pub const ENABLE_INSERT_MODE: DWORD = 0x0020;
        pub const ENABLE_QUICK_EDIT_MODE: DWORD = 0x0040;
        pub const ENABLE_EXTENDED_FLAGS: DWORD = 0x0080;
        pub const ENABLE_AUTO_POSITION: DWORD = 0x0100;
        pub const ENABLE_VIRTUAL_TERMINAL_INPUT: DWORD = 0x0200;
        pub const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001;
        pub const ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002;
        pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;

        pub const DISABLE_NEWLINE_AUTO_RETURN: DWORD = 0x0008;

        STRUCT!{struct SECURITY_ATTRIBUTES {
            nLength: DWORD,
            lpSecurityDescriptor: LPVOID,
            bInheritHandle: BOOL,
        }}

        STRUCT!{struct COORD {
            X: SHORT,
            Y: SHORT,
        }}

        pub type PSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
        pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;

        STRUCT!{struct SMALL_RECT {
            Left: SHORT,
            Top: SHORT,
            Right: SHORT,
            Bottom: SHORT,
        }}

        STRUCT!{struct CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD,
            dwCursorPosition: COORD,
            wAttributes: WORD,
            srWindow: SMALL_RECT,
            dwMaximumWindowSize: COORD,
        }}

        pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut CONSOLE_SCREEN_BUFFER_INFO;

        STRUCT!{struct CONSOLE_CURSOR_INFO {
            dwSize: DWORD,
            bVisible: BOOL,
        }}

        pub type PCONSOLE_CURSOR_INFO = *mut CONSOLE_CURSOR_INFO;

        UNION!{union CHAR_INFO_Char {
            [u16; 1],
            UnicodeChar UnicodeChar_mut: WCHAR,
            AsciiChar AsciiChar_mut: CHAR,
        }}

        STRUCT!{struct CHAR_INFO {
            Char: CHAR_INFO_Char,
            Attributes: WORD,
        }}

        #[link(name = "kernel32")]
        unsafe extern "system"
        {
            pub fn CreateConsoleScreenBuffer( dwDesiredAccess: DWORD, dwShareMode: DWORD, lpSecurityAttributes: *const SECURITY_ATTRIBUTES, dwFlags: DWORD, lpScreenBufferData: LPVOID ) -> HANDLE;
            pub fn FillConsoleOutputAttribute( hConsoleOutput: HANDLE, wAttribute: WORD, nLength: DWORD, dwWriteCoord: COORD, lpNumberOfAttrsWritten: LPDWORD ) -> BOOL;
            pub fn FillConsoleOutputCharacterA( hConsoleOutput: HANDLE, cCharacter: CHAR, nLength: DWORD, dwWriteCoord: COORD, lpNumberOfCharsWritten: LPDWORD ) -> BOOL;
            pub fn GetConsoleScreenBufferInfo( hConsoleOutput: HANDLE, lpConsoleScreenBufferInfo: PCONSOLE_SCREEN_BUFFER_INFO ) -> BOOL;
            pub fn ScrollConsoleScreenBufferW( hConsoleOutput: HANDLE, lpScrollRectangle: *const SMALL_RECT, lpClipRectangle: *const SMALL_RECT, dwDestinationOrigin: COORD, lpFill: *const CHAR_INFO ) -> BOOL;
            pub fn SetConsoleActiveScreenBuffer( hConsoleOutput: HANDLE ) -> BOOL;
            pub fn SetConsoleCursorInfo( hConsoleOutput: HANDLE, lpConsoleCursorInfo: *const CONSOLE_CURSOR_INFO ) -> BOOL;
            pub fn SetConsoleCursorPosition( hConsoleOutput: HANDLE, dwCursorPosition: COORD ) -> BOOL;
            pub fn SetConsoleScreenBufferSize( hConsoleOutput: HANDLE, dwSize: COORD ) -> BOOL;
            pub fn SetConsoleTextAttribute( hConsoleOutput: HANDLE, wAttributes: WORD ) -> BOOL;
            pub fn SetConsoleWindowInfo( hConsoleOutput: HANDLE, bAbsolute: BOOL, lpConsoleWindow: *const SMALL_RECT ) -> BOOL;
            pub fn WriteConsoleInputW( hConsoleInput: HANDLE, lpBuffer: *const INPUT_RECORD, nLength: DWORD, lpNumberOfEventsWritten: LPDWORD ) -> BOOL;
        }
    }

    pub mod winnt
    {
        use crate::
        {
            ctypes::{ * },
            *
        };

        pub const FILE_SHARE_READ: DWORD = 0x00000001;
        pub const FILE_SHARE_WRITE: DWORD = 0x00000002;
        pub const FILE_SHARE_DELETE: DWORD = 0x00000004;

        pub const GENERIC_READ: DWORD = 0x80000000;
        pub const GENERIC_WRITE: DWORD = 0x40000000;
        pub const GENERIC_EXECUTE: DWORD = 0x20000000;
        pub const GENERIC_ALL: DWORD = 0x10000000;

        pub const STATUS_WAIT_0: DWORD = 0x00000000;
        pub const STATUS_ABANDONED_WAIT_0: DWORD = 0x00000080;
        pub const STATUS_USER_APC: DWORD = 0x000000C0;
        pub const STATUS_TIMEOUT: DWORD = 0x00000102;
        pub const STATUS_PENDING: DWORD = 0x00000103;
    }

    pub mod winuser
    {
        use crate::
        {
            ctypes::{ * },
            *
        };

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

    pub const GENERIC_READ: DWORD = 0x80000000;
    pub const GENERIC_WRITE: DWORD = 0x40000000;
    pub const GENERIC_EXECUTE: DWORD = 0x20000000;
    pub const GENERIC_ALL: DWORD = 0x10000000;
}

pub mod shared
{
    use crate::
    {
        *
    };

    pub mod minwindef
    {
        use crate::
        {
            ctypes::{ * },
            *
        };

        pub type BOOL = c_int;
        pub type BYTE = c_uchar;
        pub type DWORD = c_ulong;
        pub type WORD = c_ushort;
        pub type FLOAT = c_float;

        pub const FALSE: BOOL = 0;
        pub const TRUE: BOOL = 1;
    }

    pub mod ntdef
    {
        use crate::
        {
            ctypes::{ * },
            *,
        };

        pub type CHAR = c_char;
        pub type HANDLE = *mut c_void;
        pub type SHORT = c_short;
        pub type VOID = c_void;
        pub type WCHAR = wchar_t;
    }

    pub mod wincon
    {
        use crate::
        {
            ctypes::{ * },
            *
        };

        use crate::um::wincon::INPUT_RECORD_Event;

        STRUCT!{struct INPUT_RECORD {
            EventType: WORD,
            Event: INPUT_RECORD_Event,
        }}
    }

    pub mod winerror
    {
        use crate::
        {
            ctypes::{ * },
            *
        };

        pub type HRESULT = c_long;

        pub const DXGI_ERROR_WAIT_TIMEOUT: HRESULT = 0x887A0027u32 as i32;

        pub const ERROR_ABANDONED_WAIT_0: DWORD = 735;
        pub const ERROR_ABANDONED_WAIT_63: DWORD = 736;

        pub const ERROR_WAIT_1: DWORD = 731;
        pub const ERROR_WAIT_2: DWORD = 732;
        pub const ERROR_WAIT_3: DWORD = 733;
        pub const ERROR_WAIT_63: DWORD = 734;

        pub const ERROR_WAIT_FOR_OPLOCK: DWORD = 765;

        pub const PLA_E_DCS_START_WAIT_TIMEOUT: HRESULT = 0x8030010Au32 as i32;
        pub const PLA_E_DC_START_WAIT_TIMEOUT: HRESULT = 0x8030010Bu32 as i32;
        pub const PLA_E_REPORT_WAIT_TIMEOUT: HRESULT = 0x8030010Cu32 as i32;

        pub const WAIT_TIMEOUT: DWORD = 258;

    }

    pub mod winnt
    {
        use crate::
        {
            *
        };
    }

    pub mod winuser
    {
        use crate::
        {
            *
        };
    }
}

pub mod win
{
    use crate::
    {
        *
    };
}


#[path = "windows/mod.rs"]
mod sys;

#[cfg(unix)]
pub use crate::sys::ext as unix;

#[cfg(windows)]
pub use sys::ext as windows;