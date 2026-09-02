use std;
use std::path::PathBuf;

//extern crate winapi;
use self::shared::winerror;
use self::um::knownfolders;
use self::um::combaseapi;
use self::um::shlobj;
use self::um::shtypes;
use self::um::winbase;
use self::um::winnt;

#[macro_export] macro_rules! DEFINE_GUID
{
    ( $name:ident, $l:expr, $w1:expr, $w2:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr ) =>
    {
        pub const $name: $crate::shared::guiddef::GUID = $crate::shared::guiddef::GUID
        {
            Data1: $l,
            Data2: $w1,
            Data3: $w2,
            Data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
        };
    }
}

#[macro_export] macro_rules! STRUCT
{
    (#[debug] $($rest:tt)*) =>
    ( STRUCT!{#[cfg_attr(feature = "impl-debug", derive(Debug))] $($rest)*} );

    ($(#[$attrs:meta])* struct $name:ident { $($field:ident: $ftype:ty,)+ }) =>
    (
        #[repr(C)] #[derive(Copy)] $(#[$attrs])*
        pub struct $name
        {
            $(pub $field: $ftype,)+
        }

        impl Clone for $name
        {
            #[inline] fn clone(&self) -> $name { *self }
        }

        #[cfg(feature = "impl-default")] impl Default for $name
        {
            #[inline] fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
    );
}

pub mod shared
{
    use crate::
    {
        *
    };

    pub mod guiddef
    {
        //! GUID definition
        use crate::
        {
            *
        };

        use ctypes::{c_uchar, c_ulong, c_ushort};
        STRUCT!{#[debug] struct GUID {
        Data1: c_ulong,
        Data2: c_ushort,
        Data3: c_ushort,
        Data4: [c_uchar; 8],
        }}
        pub type LPGUID = *mut GUID;
        pub type LPCGUID = *const GUID;
        pub type IID = GUID;
        pub type LPIID = *mut IID;
        pub use self::IsEqualGUID as IsEqualIID;
        pub type CLSID = GUID;
        pub type LPCLSID = *mut CLSID;
        pub use self::IsEqualGUID as IsEqualCLSID;
        pub type FMTID = GUID;
        pub type LPFMTID = *mut FMTID;
        pub use self::IsEqualGUID as IsEqualFMTID;
        pub type REFGUID = *const GUID;
        pub type REFIID = *const IID;
        pub type REFCLSID = *const IID;
        pub type REFFMTID = *const IID;
        DEFINE_GUID!{IID_NULL,
        0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
        #[inline]
        pub fn IsEqualGUID(g1: &GUID, g2: &GUID) -> bool {
            let a = unsafe { &*(g1 as *const _ as *const [u32; 4]) };
            let b = unsafe { &*(g2 as *const _ as *const [u32; 4]) };
            a[0] == b[0] && a[1] == b[1] && a[2] == b[2] && a[3] == b[3]
        }
    }

    pub mod winerror
    {
        use crate::
        {
            *
        };
    }
}

pub mod um
{
    use crate::
    {
        *
    };

    pub mod knownfolders
    {
        use crate::
        {
            *,
        };

        DEFINE_GUID!{FOLDERID_Profile, 0x5E6C858F, 0x0E22, 0x4760, 0x9A, 0xFE, 0xEA, 0x33, 0x17, 0xB6, 0x71, 0x73}
    }

    pub mod combaseapi
    {
        use crate::
        {
            *,
        };
    }

    pub mod shlobj
    {
        use crate::
        {
            *,
        };
    }

    pub mod shtypes
    {
        use crate::
        {
            *,
        };
    }

    pub mod winbase
    {
        use crate::
        {
            *,
        };
    }

    pub mod winnt
    {
        use crate::
        {
            *,
        };
    }
}

pub fn home_dir()       -> Option<PathBuf> { known_folder(&knownfolders::FOLDERID_Profile) }
pub fn data_dir()       -> Option<PathBuf> { known_folder(&knownfolders::FOLDERID_RoamingAppData) }
pub fn data_local_dir() -> Option<PathBuf> { known_folder(&knownfolders::FOLDERID_LocalAppData) }
pub fn cache_dir()      -> Option<PathBuf> { data_local_dir() }
pub fn config_dir()     -> Option<PathBuf> { data_dir() }
pub fn executable_dir() -> Option<PathBuf> { None }
pub fn runtime_dir()    -> Option<PathBuf> { None }
pub fn audio_dir()      -> Option<PathBuf> { known_folder(&knownfolders::FOLDERID_Music) }
pub fn desktop_dir()    -> Option<PathBuf> { known_folder(&knownfolders::FOLDERID_Desktop) }
pub fn document_dir()   -> Option<PathBuf> { known_folder(&knownfolders::FOLDERID_Documents) }
pub fn download_dir()   -> Option<PathBuf> { known_folder(&knownfolders::FOLDERID_Downloads) }
pub fn font_dir()       -> Option<PathBuf> { None }
pub fn picture_dir()    -> Option<PathBuf> { known_folder(&knownfolders::FOLDERID_Pictures) }
pub fn public_dir()     -> Option<PathBuf> { known_folder(&knownfolders::FOLDERID_Public) }
pub fn template_dir()   -> Option<PathBuf> { known_folder(&knownfolders::FOLDERID_Templates) }
pub fn video_dir()      -> Option<PathBuf> { known_folder(&knownfolders::FOLDERID_Videos) }

fn known_folder(folder_id: shtypes::REFKNOWNFOLDERID) -> Option<PathBuf> {
    unsafe {
        let mut path_ptr: winnt::PWSTR = std::ptr::null_mut();
        let result = shlobj::SHGetKnownFolderPath(folder_id, 0, std::ptr::null_mut(), &mut path_ptr);
        if result == winerror::S_OK {
            let len = winbase::lstrlenW(path_ptr) as usize;
            let path = std::slice::from_raw_parts(path_ptr, len);
            let ostr: std::ffi::OsString = std::os::windows::ffi::OsStringExt::from_wide(path);
            combaseapi::CoTaskMemFree(path_ptr as *mut crate::ctypes::c_void);
            Some(PathBuf::from(ostr))
        } else {
            None
        }
    }
}
