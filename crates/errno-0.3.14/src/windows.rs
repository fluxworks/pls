//! Implementation of `errno` functionality for Windows.
//!
//! Adapted from `src/libstd/sys/windows/os.rs` in the Rust distribution.

// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::char::{self, REPLACEMENT_CHARACTER};
use core::ptr;
use core::str;

pub type Win32Error = u32;
pub type FormatMessageOptions = u32;
pub type BOOL = i32;
pub type HRESULT = i32;
pub type PSTR = *mut u8;
pub type PWSTR = *mut u16;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub type BSTR = *const u16;
pub type HSTRING = *mut core::ffi::c_void;

pub const FORMAT_MESSAGE_FROM_SYSTEM: FormatMessageOptions = 4096u32;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: FormatMessageOptions = 512u32;

#[link(name = "kernel32.dll")]
unsafe extern "C"
{
    fn GetLastError() -> Win32Error;
    fn SetLastError(dwerrcode : Win32Error);
    fn FormatMessageW(dwflags : FormatMessageOptions, lpsource : *const core::ffi::c_void, dwmessageid : u32, dwlanguageid : u32, lpbuffer :PWSTR, nsize : u32, arguments : *const *const i8) -> u32;
}

use crate::Errno;

fn from_utf16_lossy<'a>(input: &[u16], output: &'a mut [u8]) -> &'a str {
    let mut output_len = 0;
    for c in char::decode_utf16(input.iter().copied().take_while(|&x| x != 0))
        .map(|x| x.unwrap_or(REPLACEMENT_CHARACTER))
    {
        let c_len = c.len_utf8();
        if c_len > output.len() - output_len {
            break;
        }
        c.encode_utf8(&mut output[output_len..]);
        output_len += c_len;
    }
    unsafe { str::from_utf8_unchecked(&output[..output_len]) }
}

pub fn with_description<F, T>(err: Errno, callback: F) -> T
where
    F: FnOnce(Result<&str, Errno>) -> T,
{
    // This value is calculated from the macro
    // MAKELANGID(LANG_SYSTEM_DEFAULT, SUBLANG_SYS_DEFAULT)
    let lang_id = 0x0800_u32;

    let mut buf = [0u16; 2048];

    unsafe {
        let res = FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null_mut(),
            err.0 as u32,
            lang_id,
            buf.as_mut_ptr(),
            buf.len() as u32,
            ptr::null_mut(),
        );
        if res == 0 {
            // Sometimes FormatMessageW can fail e.g. system doesn't like lang_id
            let fm_err = errno();
            return callback(Err(fm_err));
        }

        let mut msg = [0u8; 2048];
        let msg = from_utf16_lossy(&buf[..res as usize], &mut msg[..]);
        // Trim trailing CRLF inserted by FormatMessageW
        callback(Ok(msg.trim_end()))
    }
}

pub const STRERROR_NAME: &str = "FormatMessageW";

pub fn errno() -> Errno {
    unsafe { Errno(GetLastError() as i32) }
}

pub fn set_errno(Errno(errno): Errno) {
    unsafe { SetLastError(errno as Win32Error) }
}
