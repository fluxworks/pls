/*!
Windows API Bindings */
#![allow
(
    non_camel_case_types,
    unused_macros,
)]

#[macro_use] pub mod macros;
/*
Windows Constants */
pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;
pub const FORMAT_MESSAGE_FROM_STRING: DWORD = 0x00000400;
pub const FORMAT_MESSAGE_FROM_HMODULE: DWORD = 0x00000800;
pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: DWORD = 0x00002000;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: DWORD = 0x000000FF;
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x00000100;
/*
Windows C Types */
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
/*
Windows Platform Types */
pub const FALSE: BOOL = 0;
pub const TRUE: BOOL = 1;
pub type DWORD = c_ulong;
pub type BOOL = c_int;
pub type BYTE = c_uchar;
pub type WORD = c_ushort;
pub type FLOAT = c_float;
pub type WCHAR = wchar_t;
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
pub type INT_PTR = isize;
pub type PINT_PTR = *mut isize;
pub type UINT_PTR = usize;
pub type PUINT_PTR = *mut usize;
pub type LONG_PTR = isize;
pub type PLONG_PTR = *mut isize;
pub type ULONG_PTR = usize;
pub type PULONG_PTR = *mut usize;
pub type SHANDLE_PTR = isize;
pub type HANDLE_PTR = usize;
pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;
pub type WIN32_ERROR = u32;
pub type LPWSTR = *mut WCHAR;
pub type uintptr_t = usize;
pub type va_list = *mut c_char;
/*
Kernel32 API*/
#[link(name = "kernel32")]
extern "system"
{
    pub fn GetLastError() -> DWORD;
    pub fn SetLastError( dwErrCode: DWORD );
    pub fn FormatMessageW( dwFlags: DWORD, lpSource: LPCVOID, dwMessageId: DWORD, dwLanguageId: DWORD, lpBuffer: LPWSTR, nSize: DWORD, Arguments: *mut va_list ) -> DWORD;
}
