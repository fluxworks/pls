//! Automatic extension loading
use super::ffi;
use crate::error::{check, to_sqlite_error};
use crate::{Connection, Error, Result};
use std::ffi::{c_char, c_int};
use std::panic::catch_unwind;

/// Automatic extension initialization routine
pub type AutoExtension = fn(Connection) -> Result<()>;

/// Raw automatic extension initialization routine
pub type RawAutoExtension = unsafe extern "C" fn(
    db: *mut ffi::sqlite3,
    pz_err_msg: *mut *mut c_char,
    _: *const ffi::sqlite3_api_routines,
) -> c_int;

/// Bridge between `RawAutoExtension` and `AutoExtension`.
pub unsafe fn init_auto_extension(
    db: *mut ffi::sqlite3,
    pz_err_msg: *mut *mut c_char,
    ax: AutoExtension,
) -> c_int {
    let r = catch_unwind(|| {
        let c = Connection::from_handle(db);
        c.and_then(ax)
    })
    .unwrap_or_else(|_| Err(Error::UnwindingPanic));
    match r {
        Err(e) => to_sqlite_error(&e, pz_err_msg),
        _ => ffi::SQLITE_OK,
    }
}

/// Register au auto-extension
pub unsafe fn register_auto_extension(ax: RawAutoExtension) -> Result<()> {
    //check(ffi::sqlite3_auto_extension(Some(ax)))
    Ok( () )
}

/// Unregister the initialization routine
pub fn cancel_auto_extension(ax: RawAutoExtension) -> bool {
    unsafe
    {
        //ffi::sqlite3_cancel_auto_extension(Some(ax)) == 1
        false
    }
}

/// Disable all automatic extensions previously registered
pub fn reset_auto_extension()
{
    unsafe
    {
        ffi::sqlite3_reset_auto_extension()
    }

}
