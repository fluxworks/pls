//! Rusqlite is an ergonomic wrapper for using SQLite from Rust.
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use fallible_iterator;
pub use fallible_streaming_iterator;

use std::cell::RefCell;
use std::default::Default;
use std::result;
use std::sync::{Arc, Mutex};

use crate::ffi::{c_char, c_int, c_uint, CStr, CString};
use crate::path::Path;


#[cfg(feature = "cache")]
use crate::cache::StatementCache;
use crate::inner_connection::InnerConnection;
use crate::raw_statement::RawStatement;
use crate::types::ValueRef;

pub use crate::bind::BindIndex;
#[cfg(feature = "cache")]
pub use crate::cache::CachedStatement;
#[cfg(feature = "column_decltype")]
pub use crate::column::Column;
#[cfg(feature = "column_metadata")]
pub use crate::column::ColumnMetadata;
pub use crate::error::{to_sqlite_error, Error};
pub use crate::ffi::error::ErrorCode;
#[cfg(feature = "load_extension")]
pub use crate::load_extension_guard::LoadExtensionGuard;
pub use crate::params::{params_from_iter, Params, ParamsFromIter};
pub use crate::row::{AndThenRows, Map, MappedRows, Row, RowIndex, Rows};
pub use crate::statement::{Statement, StatementStatus};
#[cfg(feature = "modern_sqlite")]
pub use crate::transaction::TransactionState;
pub use crate::transaction::{DropBehavior, Savepoint, Transaction, TransactionBehavior};
pub use crate::types::ToSql;
pub use crate::util::Name;
pub use crate::version::*;
#[cfg(feature = "rusqlite-macros")]
#[doc(hidden)]
pub use rusqlite_macros::__bind;
///
pub mod errors
{
    pub use std::error::{ * };
}
///
pub mod fmt
{
    pub use std::fmt::{ * };
}
///
pub mod ffi
{
    /*!
    */
    pub use std::ffi::{ * };

    use crate::
    {
        *
    };
    pub mod error
    {
        /*!
        */
        use crate::
        {
            *
        };
        use crate::types::FromSqlError;
        use crate::types::Type;
        use crate::{errmsg_to_string, ffi, Result};
        use crate::ffi::{c_char, c_int, NulError};
        use crate::fmt;
        use crate::path::PathBuf;
        use crate::str;


        /// Error Codes
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum ErrorCode {
            /// Internal logic error in SQLite
            InternalMalfunction,
            /// Access permission denied
            PermissionDenied,
            /// Callback routine requested an abort
            OperationAborted,
            /// The database file is locked
            DatabaseBusy,
            /// A table in the database is locked
            DatabaseLocked,
            /// A `malloc()` failed
            OutOfMemory,
            /// Attempt to write a readonly database
            ReadOnly,
            /// Operation terminated by `sqlite3_interrupt()`
            OperationInterrupted,
            /// Some kind of disk I/O error occurred
            SystemIoFailure,
            /// The database disk image is malformed
            DatabaseCorrupt,
            /// Unknown opcode in `sqlite3_file_control()`
            NotFound,
            /// Insertion failed because database is full
            DiskFull,
            /// Unable to open the database file
            CannotOpen,
            /// Database lock protocol error
            FileLockingProtocolFailed,
            /// The database schema changed
            SchemaChanged,
            /// String or BLOB exceeds size limit
            TooBig,
            /// Abort due to constraint violation
            ConstraintViolation,
            /// Data type mismatch
            TypeMismatch,
            /// Library used incorrectly
            ApiMisuse,
            /// Uses OS features not supported on host
            NoLargeFileSupport,
            /// Authorization denied
            AuthorizationForStatementDenied,
            /// 2nd parameter to `sqlite3_bind` out of range
            ParameterOutOfRange,
            /// File opened that is not a database file
            NotADatabase,
            /// SQL error or missing database
            Unknown,
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct Error {
            pub code: ErrorCode,
            pub extended_code: c_int,
        }

        impl Error {
            #[must_use]
            pub fn new(result_code: c_int) -> Self {
                let code = match result_code & 0xff {
                    super::SQLITE_INTERNAL => ErrorCode::InternalMalfunction,
                    super::SQLITE_PERM => ErrorCode::PermissionDenied,
                    super::SQLITE_ABORT => ErrorCode::OperationAborted,
                    super::SQLITE_BUSY => ErrorCode::DatabaseBusy,
                    super::SQLITE_LOCKED => ErrorCode::DatabaseLocked,
                    super::SQLITE_NOMEM => ErrorCode::OutOfMemory,
                    super::SQLITE_READONLY => ErrorCode::ReadOnly,
                    super::SQLITE_INTERRUPT => ErrorCode::OperationInterrupted,
                    super::SQLITE_IOERR => ErrorCode::SystemIoFailure,
                    super::SQLITE_CORRUPT => ErrorCode::DatabaseCorrupt,
                    super::SQLITE_NOTFOUND => ErrorCode::NotFound,
                    super::SQLITE_FULL => ErrorCode::DiskFull,
                    super::SQLITE_CANTOPEN => ErrorCode::CannotOpen,
                    super::SQLITE_PROTOCOL => ErrorCode::FileLockingProtocolFailed,
                    super::SQLITE_SCHEMA => ErrorCode::SchemaChanged,
                    super::SQLITE_TOOBIG => ErrorCode::TooBig,
                    super::SQLITE_CONSTRAINT => ErrorCode::ConstraintViolation,
                    super::SQLITE_MISMATCH => ErrorCode::TypeMismatch,
                    super::SQLITE_MISUSE => ErrorCode::ApiMisuse,
                    super::SQLITE_NOLFS => ErrorCode::NoLargeFileSupport,
                    super::SQLITE_AUTH => ErrorCode::AuthorizationForStatementDenied,
                    super::SQLITE_RANGE => ErrorCode::ParameterOutOfRange,
                    super::SQLITE_NOTADB => ErrorCode::NotADatabase,
                    _ => ErrorCode::Unknown,
                };

                Self {
                    code,
                    extended_code: result_code,
                }
            }
        }

        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "Error code {}: {}",
                    self.extended_code,
                    code_to_str(self.extended_code)
                )
            }
        }

        impl error::Error for Error {
            fn description(&self) -> &str {
                code_to_str(self.extended_code)
            }
        }

        #[must_use]
        pub fn code_to_str(code: c_int) -> &'static str {
            let err_str = unsafe { super::sqlite3_errstr(code) };
            if err_str.is_null() {
                "Unknown errod code"
            } else {
                // We know these values to be plain ASCII
                unsafe { CStr::from_ptr(err_str) }.to_str().unwrap()
            }
        }

        /// Loadable extension initialization error
        #[cfg(feature = "loadable_extension")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum InitError {
            /// Version mismatch between the extension and the SQLite3 library
            VersionMismatch { compile_time: i32, runtime: i32 },
            /// Invalid function pointer in one of `sqlite3_api_routines` fields
            NullFunctionPointer,
        }
        #[cfg(feature = "loadable_extension")]
        impl fmt::Display for InitError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::VersionMismatch {
                        compile_time,
                        runtime,
                    } => {
                        write!(f, "SQLite version mismatch: {runtime} < {compile_time}")
                    }
                    Self::NullFunctionPointer => {
                        write!(f, "Some sqlite3_api_routines fields are null")
                    }
                }
            }
        }
        #[cfg(feature = "loadable_extension")]
        impl error::Error for InitError {}
    }
    /*
    automatically generated by rust-bindgen 0.72.1 */
    pub const SQLITE_VERSION: CStr = c"3.34.1";
    pub const SQLITE_VERSION_NUMBER: i32 = 3034001;
    pub const SQLITE_SOURCE_ID: CStr = c"2021-01-20 14:10:07 10e20c0b43500cfb9bbc0eaa061c57514f715d87238f4d835880cd846b9ebd1f";
    pub const SQLITE_OK: i32 = 0;
    pub const SQLITE_ERROR: i32 = 1;
    pub const SQLITE_INTERNAL: i32 = 2;
    pub const SQLITE_PERM: i32 = 3;
    pub const SQLITE_ABORT: i32 = 4;
    pub const SQLITE_BUSY: i32 = 5;
    pub const SQLITE_LOCKED: i32 = 6;
    pub const SQLITE_NOMEM: i32 = 7;
    pub const SQLITE_READONLY: i32 = 8;
    pub const SQLITE_INTERRUPT: i32 = 9;
    pub const SQLITE_IOERR: i32 = 10;
    pub const SQLITE_CORRUPT: i32 = 11;
    pub const SQLITE_NOTFOUND: i32 = 12;
    pub const SQLITE_FULL: i32 = 13;
    pub const SQLITE_CANTOPEN: i32 = 14;
    pub const SQLITE_PROTOCOL: i32 = 15;
    pub const SQLITE_EMPTY: i32 = 16;
    pub const SQLITE_SCHEMA: i32 = 17;
    pub const SQLITE_TOOBIG: i32 = 18;
    pub const SQLITE_CONSTRAINT: i32 = 19;
    pub const SQLITE_MISMATCH: i32 = 20;
    pub const SQLITE_MISUSE: i32 = 21;
    pub const SQLITE_NOLFS: i32 = 22;
    pub const SQLITE_AUTH: i32 = 23;
    pub const SQLITE_FORMAT: i32 = 24;
    pub const SQLITE_RANGE: i32 = 25;
    pub const SQLITE_NOTADB: i32 = 26;
    pub const SQLITE_NOTICE: i32 = 27;
    pub const SQLITE_WARNING: i32 = 28;
    pub const SQLITE_ROW: i32 = 100;
    pub const SQLITE_DONE: i32 = 101;
    pub const SQLITE_ERROR_MISSING_COLLSEQ: i32 = 257;
    pub const SQLITE_ERROR_RETRY: i32 = 513;
    pub const SQLITE_ERROR_SNAPSHOT: i32 = 769;
    pub const SQLITE_IOERR_READ: i32 = 266;
    pub const SQLITE_IOERR_SHORT_READ: i32 = 522;
    pub const SQLITE_IOERR_WRITE: i32 = 778;
    pub const SQLITE_IOERR_FSYNC: i32 = 1034;
    pub const SQLITE_IOERR_DIR_FSYNC: i32 = 1290;
    pub const SQLITE_IOERR_TRUNCATE: i32 = 1546;
    pub const SQLITE_IOERR_FSTAT: i32 = 1802;
    pub const SQLITE_IOERR_UNLOCK: i32 = 2058;
    pub const SQLITE_IOERR_RDLOCK: i32 = 2314;
    pub const SQLITE_IOERR_DELETE: i32 = 2570;
    pub const SQLITE_IOERR_BLOCKED: i32 = 2826;
    pub const SQLITE_IOERR_NOMEM: i32 = 3082;
    pub const SQLITE_IOERR_ACCESS: i32 = 3338;
    pub const SQLITE_IOERR_CHECKRESERVEDLOCK: i32 = 3594;
    pub const SQLITE_IOERR_LOCK: i32 = 3850;
    pub const SQLITE_IOERR_CLOSE: i32 = 4106;
    pub const SQLITE_IOERR_DIR_CLOSE: i32 = 4362;
    pub const SQLITE_IOERR_SHMOPEN: i32 = 4618;
    pub const SQLITE_IOERR_SHMSIZE: i32 = 4874;
    pub const SQLITE_IOERR_SHMLOCK: i32 = 5130;
    pub const SQLITE_IOERR_SHMMAP: i32 = 5386;
    pub const SQLITE_IOERR_SEEK: i32 = 5642;
    pub const SQLITE_IOERR_DELETE_NOENT: i32 = 5898;
    pub const SQLITE_IOERR_MMAP: i32 = 6154;
    pub const SQLITE_IOERR_GETTEMPPATH: i32 = 6410;
    pub const SQLITE_IOERR_CONVPATH: i32 = 6666;
    pub const SQLITE_IOERR_VNODE: i32 = 6922;
    pub const SQLITE_IOERR_AUTH: i32 = 7178;
    pub const SQLITE_IOERR_BEGIN_ATOMIC: i32 = 7434;
    pub const SQLITE_IOERR_COMMIT_ATOMIC: i32 = 7690;
    pub const SQLITE_IOERR_ROLLBACK_ATOMIC: i32 = 7946;
    pub const SQLITE_IOERR_DATA: i32 = 8202;
    pub const SQLITE_IOERR_CORRUPTFS: i32 = 8458;
    pub const SQLITE_LOCKED_SHAREDCACHE: i32 = 262;
    pub const SQLITE_LOCKED_VTAB: i32 = 518;
    pub const SQLITE_BUSY_RECOVERY: i32 = 261;
    pub const SQLITE_BUSY_SNAPSHOT: i32 = 517;
    pub const SQLITE_BUSY_TIMEOUT: i32 = 773;
    pub const SQLITE_CANTOPEN_NOTEMPDIR: i32 = 270;
    pub const SQLITE_CANTOPEN_ISDIR: i32 = 526;
    pub const SQLITE_CANTOPEN_FULLPATH: i32 = 782;
    pub const SQLITE_CANTOPEN_CONVPATH: i32 = 1038;
    pub const SQLITE_CANTOPEN_DIRTYWAL: i32 = 1294;
    pub const SQLITE_CANTOPEN_SYMLINK: i32 = 1550;
    pub const SQLITE_CORRUPT_VTAB: i32 = 267;
    pub const SQLITE_CORRUPT_SEQUENCE: i32 = 523;
    pub const SQLITE_CORRUPT_INDEX: i32 = 779;
    pub const SQLITE_READONLY_RECOVERY: i32 = 264;
    pub const SQLITE_READONLY_CANTLOCK: i32 = 520;
    pub const SQLITE_READONLY_ROLLBACK: i32 = 776;
    pub const SQLITE_READONLY_DBMOVED: i32 = 1032;
    pub const SQLITE_READONLY_CANTINIT: i32 = 1288;
    pub const SQLITE_READONLY_DIRECTORY: i32 = 1544;
    pub const SQLITE_ABORT_ROLLBACK: i32 = 516;
    pub const SQLITE_CONSTRAINT_CHECK: i32 = 275;
    pub const SQLITE_CONSTRAINT_COMMITHOOK: i32 = 531;
    pub const SQLITE_CONSTRAINT_FOREIGNKEY: i32 = 787;
    pub const SQLITE_CONSTRAINT_FUNCTION: i32 = 1043;
    pub const SQLITE_CONSTRAINT_NOTNULL: i32 = 1299;
    pub const SQLITE_CONSTRAINT_PRIMARYKEY: i32 = 1555;
    pub const SQLITE_CONSTRAINT_TRIGGER: i32 = 1811;
    pub const SQLITE_CONSTRAINT_UNIQUE: i32 = 2067;
    pub const SQLITE_CONSTRAINT_VTAB: i32 = 2323;
    pub const SQLITE_CONSTRAINT_ROWID: i32 = 2579;
    pub const SQLITE_CONSTRAINT_PINNED: i32 = 2835;
    pub const SQLITE_NOTICE_RECOVER_WAL: i32 = 283;
    pub const SQLITE_NOTICE_RECOVER_ROLLBACK: i32 = 539;
    pub const SQLITE_WARNING_AUTOINDEX: i32 = 284;
    pub const SQLITE_AUTH_USER: i32 = 279;
    pub const SQLITE_OK_LOAD_PERMANENTLY: i32 = 256;
    pub const SQLITE_OK_SYMLINK: i32 = 512;
    pub const SQLITE_OPEN_READONLY: i32 = 1;
    pub const SQLITE_OPEN_READWRITE: i32 = 2;
    pub const SQLITE_OPEN_CREATE: i32 = 4;
    pub const SQLITE_OPEN_DELETEONCLOSE: i32 = 8;
    pub const SQLITE_OPEN_EXCLUSIVE: i32 = 16;
    pub const SQLITE_OPEN_AUTOPROXY: i32 = 32;
    pub const SQLITE_OPEN_URI: i32 = 64;
    pub const SQLITE_OPEN_MEMORY: i32 = 128;
    pub const SQLITE_OPEN_MAIN_DB: i32 = 256;
    pub const SQLITE_OPEN_TEMP_DB: i32 = 512;
    pub const SQLITE_OPEN_TRANSIENT_DB: i32 = 1024;
    pub const SQLITE_OPEN_MAIN_JOURNAL: i32 = 2048;
    pub const SQLITE_OPEN_TEMP_JOURNAL: i32 = 4096;
    pub const SQLITE_OPEN_SUBJOURNAL: i32 = 8192;
    pub const SQLITE_OPEN_SUPER_JOURNAL: i32 = 16384;
    pub const SQLITE_OPEN_NOMUTEX: i32 = 32768;
    pub const SQLITE_OPEN_FULLMUTEX: i32 = 65536;
    pub const SQLITE_OPEN_SHAREDCACHE: i32 = 131072;
    pub const SQLITE_OPEN_PRIVATECACHE: i32 = 262144;
    pub const SQLITE_OPEN_WAL: i32 = 524288;
    pub const SQLITE_OPEN_NOFOLLOW: i32 = 16777216;
    pub const SQLITE_OPEN_MASTER_JOURNAL: i32 = 16384;
    pub const SQLITE_IOCAP_ATOMIC: i32 = 1;
    pub const SQLITE_IOCAP_ATOMIC512: i32 = 2;
    pub const SQLITE_IOCAP_ATOMIC1K: i32 = 4;
    pub const SQLITE_IOCAP_ATOMIC2K: i32 = 8;
    pub const SQLITE_IOCAP_ATOMIC4K: i32 = 16;
    pub const SQLITE_IOCAP_ATOMIC8K: i32 = 32;
    pub const SQLITE_IOCAP_ATOMIC16K: i32 = 64;
    pub const SQLITE_IOCAP_ATOMIC32K: i32 = 128;
    pub const SQLITE_IOCAP_ATOMIC64K: i32 = 256;
    pub const SQLITE_IOCAP_SAFE_APPEND: i32 = 512;
    pub const SQLITE_IOCAP_SEQUENTIAL: i32 = 1024;
    pub const SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN: i32 = 2048;
    pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE: i32 = 4096;
    pub const SQLITE_IOCAP_IMMUTABLE: i32 = 8192;
    pub const SQLITE_IOCAP_BATCH_ATOMIC: i32 = 16384;
    pub const SQLITE_LOCK_NONE: i32 = 0;
    pub const SQLITE_LOCK_SHARED: i32 = 1;
    pub const SQLITE_LOCK_RESERVED: i32 = 2;
    pub const SQLITE_LOCK_PENDING: i32 = 3;
    pub const SQLITE_LOCK_EXCLUSIVE: i32 = 4;
    pub const SQLITE_SYNC_NORMAL: i32 = 2;
    pub const SQLITE_SYNC_FULL: i32 = 3;
    pub const SQLITE_SYNC_DATAONLY: i32 = 16;
    pub const SQLITE_FCNTL_LOCKSTATE: i32 = 1;
    pub const SQLITE_FCNTL_GET_LOCKPROXYFILE: i32 = 2;
    pub const SQLITE_FCNTL_SET_LOCKPROXYFILE: i32 = 3;
    pub const SQLITE_FCNTL_LAST_ERRNO: i32 = 4;
    pub const SQLITE_FCNTL_SIZE_HINT: i32 = 5;
    pub const SQLITE_FCNTL_CHUNK_SIZE: i32 = 6;
    pub const SQLITE_FCNTL_FILE_POINTER: i32 = 7;
    pub const SQLITE_FCNTL_SYNC_OMITTED: i32 = 8;
    pub const SQLITE_FCNTL_WIN32_AV_RETRY: i32 = 9;
    pub const SQLITE_FCNTL_PERSIST_WAL: i32 = 10;
    pub const SQLITE_FCNTL_OVERWRITE: i32 = 11;
    pub const SQLITE_FCNTL_VFSNAME: i32 = 12;
    pub const SQLITE_FCNTL_POWERSAFE_OVERWRITE: i32 = 13;
    pub const SQLITE_FCNTL_PRAGMA: i32 = 14;
    pub const SQLITE_FCNTL_BUSYHANDLER: i32 = 15;
    pub const SQLITE_FCNTL_TEMPFILENAME: i32 = 16;
    pub const SQLITE_FCNTL_MMAP_SIZE: i32 = 18;
    pub const SQLITE_FCNTL_TRACE: i32 = 19;
    pub const SQLITE_FCNTL_HAS_MOVED: i32 = 20;
    pub const SQLITE_FCNTL_SYNC: i32 = 21;
    pub const SQLITE_FCNTL_COMMIT_PHASETWO: i32 = 22;
    pub const SQLITE_FCNTL_WIN32_SET_HANDLE: i32 = 23;
    pub const SQLITE_FCNTL_WAL_BLOCK: i32 = 24;
    pub const SQLITE_FCNTL_ZIPVFS: i32 = 25;
    pub const SQLITE_FCNTL_RBU: i32 = 26;
    pub const SQLITE_FCNTL_VFS_POINTER: i32 = 27;
    pub const SQLITE_FCNTL_JOURNAL_POINTER: i32 = 28;
    pub const SQLITE_FCNTL_WIN32_GET_HANDLE: i32 = 29;
    pub const SQLITE_FCNTL_PDB: i32 = 30;
    pub const SQLITE_FCNTL_BEGIN_ATOMIC_WRITE: i32 = 31;
    pub const SQLITE_FCNTL_COMMIT_ATOMIC_WRITE: i32 = 32;
    pub const SQLITE_FCNTL_ROLLBACK_ATOMIC_WRITE: i32 = 33;
    pub const SQLITE_FCNTL_LOCK_TIMEOUT: i32 = 34;
    pub const SQLITE_FCNTL_DATA_VERSION: i32 = 35;
    pub const SQLITE_FCNTL_SIZE_LIMIT: i32 = 36;
    pub const SQLITE_FCNTL_CKPT_DONE: i32 = 37;
    pub const SQLITE_FCNTL_RESERVE_BYTES: i32 = 38;
    pub const SQLITE_FCNTL_CKPT_START: i32 = 39;
    pub const SQLITE_GET_LOCKPROXYFILE: i32 = 2;
    pub const SQLITE_SET_LOCKPROXYFILE: i32 = 3;
    pub const SQLITE_LAST_ERRNO: i32 = 4;
    pub const SQLITE_ACCESS_EXISTS: i32 = 0;
    pub const SQLITE_ACCESS_READWRITE: i32 = 1;
    pub const SQLITE_ACCESS_READ: i32 = 2;
    pub const SQLITE_SHM_UNLOCK: i32 = 1;
    pub const SQLITE_SHM_LOCK: i32 = 2;
    pub const SQLITE_SHM_SHARED: i32 = 4;
    pub const SQLITE_SHM_EXCLUSIVE: i32 = 8;
    pub const SQLITE_SHM_NLOCK: i32 = 8;
    pub const SQLITE_CONFIG_SINGLETHREAD: i32 = 1;
    pub const SQLITE_CONFIG_MULTITHREAD: i32 = 2;
    pub const SQLITE_CONFIG_SERIALIZED: i32 = 3;
    pub const SQLITE_CONFIG_MALLOC: i32 = 4;
    pub const SQLITE_CONFIG_GETMALLOC: i32 = 5;
    pub const SQLITE_CONFIG_SCRATCH: i32 = 6;
    pub const SQLITE_CONFIG_PAGECACHE: i32 = 7;
    pub const SQLITE_CONFIG_HEAP: i32 = 8;
    pub const SQLITE_CONFIG_MEMSTATUS: i32 = 9;
    pub const SQLITE_CONFIG_MUTEX: i32 = 10;
    pub const SQLITE_CONFIG_GETMUTEX: i32 = 11;
    pub const SQLITE_CONFIG_LOOKASIDE: i32 = 13;
    pub const SQLITE_CONFIG_PCACHE: i32 = 14;
    pub const SQLITE_CONFIG_GETPCACHE: i32 = 15;
    pub const SQLITE_CONFIG_LOG: i32 = 16;
    pub const SQLITE_CONFIG_URI: i32 = 17;
    pub const SQLITE_CONFIG_PCACHE2: i32 = 18;
    pub const SQLITE_CONFIG_GETPCACHE2: i32 = 19;
    pub const SQLITE_CONFIG_COVERING_INDEX_SCAN: i32 = 20;
    pub const SQLITE_CONFIG_SQLLOG: i32 = 21;
    pub const SQLITE_CONFIG_MMAP_SIZE: i32 = 22;
    pub const SQLITE_CONFIG_WIN32_HEAPSIZE: i32 = 23;
    pub const SQLITE_CONFIG_PCACHE_HDRSZ: i32 = 24;
    pub const SQLITE_CONFIG_PMASZ: i32 = 25;
    pub const SQLITE_CONFIG_STMTJRNL_SPILL: i32 = 26;
    pub const SQLITE_CONFIG_SMALL_MALLOC: i32 = 27;
    pub const SQLITE_CONFIG_SORTERREF_SIZE: i32 = 28;
    pub const SQLITE_CONFIG_MEMDB_MAXSIZE: i32 = 29;
    pub const SQLITE_DBCONFIG_MAINDBNAME: i32 = 1000;
    pub const SQLITE_DBCONFIG_LOOKASIDE: i32 = 1001;
    pub const SQLITE_DBCONFIG_ENABLE_FKEY: i32 = 1002;
    pub const SQLITE_DBCONFIG_ENABLE_TRIGGER: i32 = 1003;
    pub const SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER: i32 = 1004;
    pub const SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION: i32 = 1005;
    pub const SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE: i32 = 1006;
    pub const SQLITE_DBCONFIG_ENABLE_QPSG: i32 = 1007;
    pub const SQLITE_DBCONFIG_TRIGGER_EQP: i32 = 1008;
    pub const SQLITE_DBCONFIG_RESET_DATABASE: i32 = 1009;
    pub const SQLITE_DBCONFIG_DEFENSIVE: i32 = 1010;
    pub const SQLITE_DBCONFIG_WRITABLE_SCHEMA: i32 = 1011;
    pub const SQLITE_DBCONFIG_LEGACY_ALTER_TABLE: i32 = 1012;
    pub const SQLITE_DBCONFIG_DQS_DML: i32 = 1013;
    pub const SQLITE_DBCONFIG_DQS_DDL: i32 = 1014;
    pub const SQLITE_DBCONFIG_ENABLE_VIEW: i32 = 1015;
    pub const SQLITE_DBCONFIG_LEGACY_FILE_FORMAT: i32 = 1016;
    pub const SQLITE_DBCONFIG_TRUSTED_SCHEMA: i32 = 1017;
    pub const SQLITE_DBCONFIG_MAX: i32 = 1017;
    pub const SQLITE_DENY: i32 = 1;
    pub const SQLITE_IGNORE: i32 = 2;
    pub const SQLITE_CREATE_INDEX: i32 = 1;
    pub const SQLITE_CREATE_TABLE: i32 = 2;
    pub const SQLITE_CREATE_TEMP_INDEX: i32 = 3;
    pub const SQLITE_CREATE_TEMP_TABLE: i32 = 4;
    pub const SQLITE_CREATE_TEMP_TRIGGER: i32 = 5;
    pub const SQLITE_CREATE_TEMP_VIEW: i32 = 6;
    pub const SQLITE_CREATE_TRIGGER: i32 = 7;
    pub const SQLITE_CREATE_VIEW: i32 = 8;
    pub const SQLITE_DELETE: i32 = 9;
    pub const SQLITE_DROP_INDEX: i32 = 10;
    pub const SQLITE_DROP_TABLE: i32 = 11;
    pub const SQLITE_DROP_TEMP_INDEX: i32 = 12;
    pub const SQLITE_DROP_TEMP_TABLE: i32 = 13;
    pub const SQLITE_DROP_TEMP_TRIGGER: i32 = 14;
    pub const SQLITE_DROP_TEMP_VIEW: i32 = 15;
    pub const SQLITE_DROP_TRIGGER: i32 = 16;
    pub const SQLITE_DROP_VIEW: i32 = 17;
    pub const SQLITE_INSERT: i32 = 18;
    pub const SQLITE_PRAGMA: i32 = 19;
    pub const SQLITE_READ: i32 = 20;
    pub const SQLITE_SELECT: i32 = 21;
    pub const SQLITE_TRANSACTION: i32 = 22;
    pub const SQLITE_UPDATE: i32 = 23;
    pub const SQLITE_ATTACH: i32 = 24;
    pub const SQLITE_DETACH: i32 = 25;
    pub const SQLITE_ALTER_TABLE: i32 = 26;
    pub const SQLITE_REINDEX: i32 = 27;
    pub const SQLITE_ANALYZE: i32 = 28;
    pub const SQLITE_CREATE_VTABLE: i32 = 29;
    pub const SQLITE_DROP_VTABLE: i32 = 30;
    pub const SQLITE_FUNCTION: i32 = 31;
    pub const SQLITE_SAVEPOINT: i32 = 32;
    pub const SQLITE_COPY: i32 = 0;
    pub const SQLITE_RECURSIVE: i32 = 33;
    pub const SQLITE_TRACE_STMT: c_uint = 1;
    pub const SQLITE_TRACE_PROFILE: c_uint = 2;
    pub const SQLITE_TRACE_ROW: c_uint = 4;
    pub const SQLITE_TRACE_CLOSE: c_uint = 8;
    pub const SQLITE_LIMIT_LENGTH: i32 = 0;
    pub const SQLITE_LIMIT_SQL_LENGTH: i32 = 1;
    pub const SQLITE_LIMIT_COLUMN: i32 = 2;
    pub const SQLITE_LIMIT_EXPR_DEPTH: i32 = 3;
    pub const SQLITE_LIMIT_COMPOUND_SELECT: i32 = 4;
    pub const SQLITE_LIMIT_VDBE_OP: i32 = 5;
    pub const SQLITE_LIMIT_FUNCTION_ARG: i32 = 6;
    pub const SQLITE_LIMIT_ATTACHED: i32 = 7;
    pub const SQLITE_LIMIT_LIKE_PATTERN_LENGTH: i32 = 8;
    pub const SQLITE_LIMIT_VARIABLE_NUMBER: i32 = 9;
    pub const SQLITE_LIMIT_TRIGGER_DEPTH: i32 = 10;
    pub const SQLITE_LIMIT_WORKER_THREADS: i32 = 11;
    pub const SQLITE_PREPARE_PERSISTENT: c_uint = 1;
    pub const SQLITE_PREPARE_NORMALIZE: c_uint = 2;
    pub const SQLITE_PREPARE_NO_VTAB: c_uint = 4;
    pub const SQLITE_INTEGER: i32 = 1;
    pub const SQLITE_FLOAT: i32 = 2;
    pub const SQLITE_BLOB: i32 = 4;
    pub const SQLITE_NULL: i32 = 5;
    pub const SQLITE_TEXT: i32 = 3;
    pub const SQLITE3_TEXT: i32 = 3;
    pub const SQLITE_UTF8: i32 = 1;
    pub const SQLITE_UTF16LE: i32 = 2;
    pub const SQLITE_UTF16BE: i32 = 3;
    pub const SQLITE_UTF16: i32 = 4;
    pub const SQLITE_ANY: i32 = 5;
    pub const SQLITE_UTF16_ALIGNED: i32 = 8;
    pub const SQLITE_DETERMINISTIC: i32 = 2048;
    pub const SQLITE_DIRECTONLY: i32 = 524288;
    pub const SQLITE_SUBTYPE: i32 = 1048576;
    pub const SQLITE_INNOCUOUS: i32 = 2097152;
    pub const SQLITE_WIN32_DATA_DIRECTORY_TYPE: i32 = 1;
    pub const SQLITE_WIN32_TEMP_DIRECTORY_TYPE: i32 = 2;
    pub const SQLITE_TXN_NONE: i32 = 0;
    pub const SQLITE_TXN_READ: i32 = 1;
    pub const SQLITE_TXN_WRITE: i32 = 2;
    pub const SQLITE_INDEX_SCAN_UNIQUE: i32 = 1;
    pub const SQLITE_INDEX_CONSTRAINT_EQ: i32 = 2;
    pub const SQLITE_INDEX_CONSTRAINT_GT: i32 = 4;
    pub const SQLITE_INDEX_CONSTRAINT_LE: i32 = 8;
    pub const SQLITE_INDEX_CONSTRAINT_LT: i32 = 16;
    pub const SQLITE_INDEX_CONSTRAINT_GE: i32 = 32;
    pub const SQLITE_INDEX_CONSTRAINT_MATCH: i32 = 64;
    pub const SQLITE_INDEX_CONSTRAINT_LIKE: i32 = 65;
    pub const SQLITE_INDEX_CONSTRAINT_GLOB: i32 = 66;
    pub const SQLITE_INDEX_CONSTRAINT_REGEXP: i32 = 67;
    pub const SQLITE_INDEX_CONSTRAINT_NE: i32 = 68;
    pub const SQLITE_INDEX_CONSTRAINT_ISNOT: i32 = 69;
    pub const SQLITE_INDEX_CONSTRAINT_ISNOTNULL: i32 = 70;
    pub const SQLITE_INDEX_CONSTRAINT_ISNULL: i32 = 71;
    pub const SQLITE_INDEX_CONSTRAINT_IS: i32 = 72;
    pub const SQLITE_INDEX_CONSTRAINT_FUNCTION: i32 = 150;
    pub const SQLITE_MUTEX_FAST: i32 = 0;
    pub const SQLITE_MUTEX_RECURSIVE: i32 = 1;
    pub const SQLITE_MUTEX_STATIC_MAIN: i32 = 2;
    pub const SQLITE_MUTEX_STATIC_MEM: i32 = 3;
    pub const SQLITE_MUTEX_STATIC_MEM2: i32 = 4;
    pub const SQLITE_MUTEX_STATIC_OPEN: i32 = 4;
    pub const SQLITE_MUTEX_STATIC_PRNG: i32 = 5;
    pub const SQLITE_MUTEX_STATIC_LRU: i32 = 6;
    pub const SQLITE_MUTEX_STATIC_LRU2: i32 = 7;
    pub const SQLITE_MUTEX_STATIC_PMEM: i32 = 7;
    pub const SQLITE_MUTEX_STATIC_APP1: i32 = 8;
    pub const SQLITE_MUTEX_STATIC_APP2: i32 = 9;
    pub const SQLITE_MUTEX_STATIC_APP3: i32 = 10;
    pub const SQLITE_MUTEX_STATIC_VFS1: i32 = 11;
    pub const SQLITE_MUTEX_STATIC_VFS2: i32 = 12;
    pub const SQLITE_MUTEX_STATIC_VFS3: i32 = 13;
    pub const SQLITE_MUTEX_STATIC_MASTER: i32 = 2;
    pub const SQLITE_TESTCTRL_FIRST: i32 = 5;
    pub const SQLITE_TESTCTRL_PRNG_SAVE: i32 = 5;
    pub const SQLITE_TESTCTRL_PRNG_RESTORE: i32 = 6;
    pub const SQLITE_TESTCTRL_PRNG_RESET: i32 = 7;
    pub const SQLITE_TESTCTRL_BITVEC_TEST: i32 = 8;
    pub const SQLITE_TESTCTRL_FAULT_INSTALL: i32 = 9;
    pub const SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS: i32 = 10;
    pub const SQLITE_TESTCTRL_PENDING_BYTE: i32 = 11;
    pub const SQLITE_TESTCTRL_ASSERT: i32 = 12;
    pub const SQLITE_TESTCTRL_ALWAYS: i32 = 13;
    pub const SQLITE_TESTCTRL_RESERVE: i32 = 14;
    pub const SQLITE_TESTCTRL_OPTIMIZATIONS: i32 = 15;
    pub const SQLITE_TESTCTRL_ISKEYWORD: i32 = 16;
    pub const SQLITE_TESTCTRL_SCRATCHMALLOC: i32 = 17;
    pub const SQLITE_TESTCTRL_INTERNAL_FUNCTIONS: i32 = 17;
    pub const SQLITE_TESTCTRL_LOCALTIME_FAULT: i32 = 18;
    pub const SQLITE_TESTCTRL_EXPLAIN_STMT: i32 = 19;
    pub const SQLITE_TESTCTRL_ONCE_RESET_THRESHOLD: i32 = 19;
    pub const SQLITE_TESTCTRL_NEVER_CORRUPT: i32 = 20;
    pub const SQLITE_TESTCTRL_VDBE_COVERAGE: i32 = 21;
    pub const SQLITE_TESTCTRL_BYTEORDER: i32 = 22;
    pub const SQLITE_TESTCTRL_ISINIT: i32 = 23;
    pub const SQLITE_TESTCTRL_SORTER_MMAP: i32 = 24;
    pub const SQLITE_TESTCTRL_IMPOSTER: i32 = 25;
    pub const SQLITE_TESTCTRL_PARSER_COVERAGE: i32 = 26;
    pub const SQLITE_TESTCTRL_RESULT_INTREAL: i32 = 27;
    pub const SQLITE_TESTCTRL_PRNG_SEED: i32 = 28;
    pub const SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS: i32 = 29;
    pub const SQLITE_TESTCTRL_SEEK_COUNT: i32 = 30;
    pub const SQLITE_TESTCTRL_LAST: i32 = 30;
    pub const SQLITE_STATUS_MEMORY_USED: i32 = 0;
    pub const SQLITE_STATUS_PAGECACHE_USED: i32 = 1;
    pub const SQLITE_STATUS_PAGECACHE_OVERFLOW: i32 = 2;
    pub const SQLITE_STATUS_SCRATCH_USED: i32 = 3;
    pub const SQLITE_STATUS_SCRATCH_OVERFLOW: i32 = 4;
    pub const SQLITE_STATUS_MALLOC_SIZE: i32 = 5;
    pub const SQLITE_STATUS_PARSER_STACK: i32 = 6;
    pub const SQLITE_STATUS_PAGECACHE_SIZE: i32 = 7;
    pub const SQLITE_STATUS_SCRATCH_SIZE: i32 = 8;
    pub const SQLITE_STATUS_MALLOC_COUNT: i32 = 9;
    pub const SQLITE_DBSTATUS_LOOKASIDE_USED: i32 = 0;
    pub const SQLITE_DBSTATUS_CACHE_USED: i32 = 1;
    pub const SQLITE_DBSTATUS_SCHEMA_USED: i32 = 2;
    pub const SQLITE_DBSTATUS_STMT_USED: i32 = 3;
    pub const SQLITE_DBSTATUS_LOOKASIDE_HIT: i32 = 4;
    pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE: i32 = 5;
    pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL: i32 = 6;
    pub const SQLITE_DBSTATUS_CACHE_HIT: i32 = 7;
    pub const SQLITE_DBSTATUS_CACHE_MISS: i32 = 8;
    pub const SQLITE_DBSTATUS_CACHE_WRITE: i32 = 9;
    pub const SQLITE_DBSTATUS_DEFERRED_FKS: i32 = 10;
    pub const SQLITE_DBSTATUS_CACHE_USED_SHARED: i32 = 11;
    pub const SQLITE_DBSTATUS_CACHE_SPILL: i32 = 12;
    pub const SQLITE_DBSTATUS_MAX: i32 = 12;
    pub const SQLITE_STMTSTATUS_FULLSCAN_STEP: i32 = 1;
    pub const SQLITE_STMTSTATUS_SORT: i32 = 2;
    pub const SQLITE_STMTSTATUS_AUTOINDEX: i32 = 3;
    pub const SQLITE_STMTSTATUS_VM_STEP: i32 = 4;
    pub const SQLITE_STMTSTATUS_REPREPARE: i32 = 5;
    pub const SQLITE_STMTSTATUS_RUN: i32 = 6;
    pub const SQLITE_STMTSTATUS_MEMUSED: i32 = 99;
    pub const SQLITE_CHECKPOINT_PASSIVE: i32 = 0;
    pub const SQLITE_CHECKPOINT_FULL: i32 = 1;
    pub const SQLITE_CHECKPOINT_RESTART: i32 = 2;
    pub const SQLITE_CHECKPOINT_TRUNCATE: i32 = 3;
    pub const SQLITE_VTAB_CONSTRAINT_SUPPORT: i32 = 1;
    pub const SQLITE_VTAB_INNOCUOUS: i32 = 2;
    pub const SQLITE_VTAB_DIRECTONLY: i32 = 3;
    pub const SQLITE_ROLLBACK: i32 = 1;
    pub const SQLITE_FAIL: i32 = 3;
    pub const SQLITE_REPLACE: i32 = 5;
    pub const SQLITE_SCANSTAT_NLOOP: i32 = 0;
    pub const SQLITE_SCANSTAT_NVISIT: i32 = 1;
    pub const SQLITE_SCANSTAT_EST: i32 = 2;
    pub const SQLITE_SCANSTAT_NAME: i32 = 3;
    pub const SQLITE_SCANSTAT_EXPLAIN: i32 = 4;
    pub const SQLITE_SCANSTAT_SELECTID: i32 = 5;
    pub const SQLITE_SERIALIZE_NOCOPY: c_uint = 1;
    pub const SQLITE_DESERIALIZE_FREEONCLOSE: c_uint = 1;
    pub const SQLITE_DESERIALIZE_RESIZEABLE: c_uint = 2;
    pub const SQLITE_DESERIALIZE_READONLY: c_uint = 4;
    pub const NOT_WITHIN: i32 = 0;
    pub const PARTLY_WITHIN: i32 = 1;
    pub const FULLY_WITHIN: i32 = 2;
    pub const FTS5_TOKENIZE_QUERY: i32 = 1;
    pub const FTS5_TOKENIZE_PREFIX: i32 = 2;
    pub const FTS5_TOKENIZE_DOCUMENT: i32 = 4;
    pub const FTS5_TOKENIZE_AUX: i32 = 8;
    pub const FTS5_TOKEN_COLOCATED: i32 = 1;
    #[link(name = "mylib", kind = "dylib")]
    unsafe extern "C"
    {
        pub static sqlite3_version: [c_char; 0usize];
    }
    
    #[repr(C)] #[derive(Debug, Copy, Clone)]
    pub struct sqlite3
    {
        _unused: [u8; 0],
    }
    
    pub type sqlite_int64 = c_longlong;
    pub type sqlite_uint64 = c_ulonglong;
    pub type sqlite3_int64 = sqlite_int64;
    pub type sqlite3_uint64 = sqlite_uint64;
    pub type sqlite3_callback = Option<
        unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: c_int,
            arg3: *mut *mut c_char,
            arg4: *mut *mut c_char,
        ) -> c_int,
    >;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_file {
        pub pMethods: *const sqlite3_io_methods,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_io_methods {
        pub iVersion: c_int,
        pub xClose:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_file) -> c_int>,
        pub xRead: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                arg2: *mut c_void,
                iAmt: c_int,
                iOfst: sqlite3_int64,
            ) -> c_int,
        >,
        pub xWrite: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                arg2: *const crate::ffi::c_void,
                iAmt: c_int,
                iOfst: sqlite3_int64,
            ) -> c_int,
        >,
        pub xTruncate: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_file, size: sqlite3_int64) -> c_int,
        >,
        pub xSync: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                flags: c_int,
            ) -> c_int,
        >,
        pub xFileSize: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                pSize: *mut sqlite3_int64,
            ) -> c_int,
        >,
        pub xLock: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                arg2: c_int,
            ) -> c_int,
        >,
        pub xUnlock: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                arg2: c_int,
            ) -> c_int,
        >,
        pub xCheckReservedLock: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                pResOut: *mut c_int,
            ) -> c_int,
        >,
        pub xFileControl: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                op: c_int,
                pArg: *mut c_void,
            ) -> c_int,
        >,
        pub xSectorSize:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_file) -> c_int>,
        pub xDeviceCharacteristics:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_file) -> c_int>,
        pub xShmMap: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                iPg: c_int,
                pgsz: c_int,
                arg2: c_int,
                arg3: *mut *mut c_void,
            ) -> c_int,
        >,
        pub xShmLock: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                offset: c_int,
                n: c_int,
                flags: c_int,
            ) -> c_int,
        >,
        pub xShmBarrier: Option<unsafe extern "C" fn(arg1: *mut sqlite3_file)>,
        pub xShmUnmap: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                deleteFlag: c_int,
            ) -> c_int,
        >,
        pub xFetch: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                iOfst: sqlite3_int64,
                iAmt: c_int,
                pp: *mut *mut c_void,
            ) -> c_int,
        >,
        pub xUnfetch: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_file,
                iOfst: sqlite3_int64,
                p: *mut c_void,
            ) -> c_int,
        >,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_mutex {
        _unused: [u8; 0],
    }
    pub type sqlite3_syscall_ptr = Option<unsafe extern "C" fn()>;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_vfs {
        pub iVersion: c_int,
        pub szOsFile: c_int,
        pub mxPathname: c_int,
        pub pNext: *mut sqlite3_vfs,
        pub zName: *const c_char,
        pub pAppData: *mut c_void,
        pub xOpen: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                zName: *const c_char,
                arg2: *mut sqlite3_file,
                flags: c_int,
                pOutFlags: *mut c_int,
            ) -> c_int,
        >,
        pub xDelete: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                zName: *const c_char,
                syncDir: c_int,
            ) -> c_int,
        >,
        pub xAccess: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                zName: *const c_char,
                flags: c_int,
                pResOut: *mut c_int,
            ) -> c_int,
        >,
        pub xFullPathname: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                zName: *const c_char,
                nOut: c_int,
                zOut: *mut c_char,
            ) -> c_int,
        >,
        pub xDlOpen: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                zFilename: *const c_char,
            ) -> *mut c_void,
        >,
        pub xDlError: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                nByte: c_int,
                zErrMsg: *mut c_char,
            ),
        >,
        pub xDlSym: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                arg2: *mut c_void,
                zSymbol: *const c_char,
            ) -> Option<
                unsafe extern "C" fn(
                    arg1: *mut sqlite3_vfs,
                    arg2: *mut c_void,
                    zSymbol: *const c_char,
                ),
            >,
        >,
        pub xDlClose: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_vfs, arg2: *mut c_void),
        >,
        pub xRandomness: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                nByte: c_int,
                zOut: *mut c_char,
            ) -> c_int,
        >,
        pub xSleep: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                microseconds: c_int,
            ) -> c_int,
        >,
        pub xCurrentTime: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_vfs, arg2: *mut f64) -> c_int,
        >,
        pub xGetLastError: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                arg2: c_int,
                arg3: *mut c_char,
            ) -> c_int,
        >,
        pub xCurrentTimeInt64: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                arg2: *mut sqlite3_int64,
            ) -> c_int,
        >,
        pub xSetSystemCall: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                zName: *const c_char,
                arg2: sqlite3_syscall_ptr,
            ) -> c_int,
        >,
        pub xGetSystemCall: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                zName: *const c_char,
            ) -> sqlite3_syscall_ptr,
        >,
        pub xNextSystemCall: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                zName: *const c_char,
            ) -> *const c_char,
        >,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_mem_methods {
        pub xMalloc: Option<
            unsafe extern "C" fn(arg1: crate::ffi::c_int) -> *mut c_void,
        >,
        pub xFree: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        pub xRealloc: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: c_int,
            ) -> *mut c_void,
        >,
        pub xSize: Option<
            unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
        >,
        pub xRoundup: Option<
            unsafe extern "C" fn(arg1: crate::ffi::c_int) -> c_int,
        >,
        pub xInit: Option<
            unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
        >,
        pub xShutdown: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        pub pAppData: *mut c_void,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_stmt {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_value {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_context {
        _unused: [u8; 0],
    }
    pub type sqlite3_destructor_type =
    Option<unsafe extern "C" fn(arg1: *mut c_void)>;
    unsafe extern "C" {
        pub static mut sqlite3_temp_directory: *mut crate::ffi::c_char;
    }
    unsafe extern "C" {
        pub static mut sqlite3_data_directory: *mut crate::ffi::c_char;
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_module {
        pub iVersion: c_int,
        pub xCreate: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                pAux: *mut c_void,
                argc: c_int,
                argv: *const *const c_char,
                ppVTab: *mut *mut sqlite3_vtab,
                arg2: *mut *mut c_char,
            ) -> c_int,
        >,
        pub xConnect: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                pAux: *mut c_void,
                argc: c_int,
                argv: *const *const c_char,
                ppVTab: *mut *mut sqlite3_vtab,
                arg2: *mut *mut c_char,
            ) -> c_int,
        >,
        pub xBestIndex: Option<
            unsafe extern "C" fn(
                pVTab: *mut sqlite3_vtab,
                arg1: *mut sqlite3_index_info,
            ) -> c_int,
        >,
        pub xDisconnect: Option<
            unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> c_int,
        >,
        pub xDestroy: Option<
            unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> c_int,
        >,
        pub xOpen: Option<
            unsafe extern "C" fn(
                pVTab: *mut sqlite3_vtab,
                ppCursor: *mut *mut sqlite3_vtab_cursor,
            ) -> c_int,
        >,
        pub xClose: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_vtab_cursor) -> c_int,
        >,
        pub xFilter: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vtab_cursor,
                idxNum: c_int,
                idxStr: *const c_char,
                argc: c_int,
                argv: *mut *mut sqlite3_value,
            ) -> c_int,
        >,
        pub xNext: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_vtab_cursor) -> c_int,
        >,
        pub xEof: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_vtab_cursor) -> c_int,
        >,
        pub xColumn: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vtab_cursor,
                arg2: *mut sqlite3_context,
                arg3: c_int,
            ) -> c_int,
        >,
        pub xRowid: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vtab_cursor,
                pRowid: *mut sqlite3_int64,
            ) -> c_int,
        >,
        pub xUpdate: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vtab,
                arg2: c_int,
                arg3: *mut *mut sqlite3_value,
                arg4: *mut sqlite3_int64,
            ) -> c_int,
        >,
        pub xBegin: Option<
            unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> c_int,
        >,
        pub xSync: Option<
            unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> c_int,
        >,
        pub xCommit: Option<
            unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> c_int,
        >,
        pub xRollback: Option<
            unsafe extern "C" fn(pVTab: *mut sqlite3_vtab) -> c_int,
        >,
        pub xFindFunction: Option<
            unsafe extern "C" fn(
                pVtab: *mut sqlite3_vtab,
                nArg: c_int,
                zName: *const c_char,
                pxFunc: *mut Option<
                    unsafe extern "C" fn(
                        arg1: *mut sqlite3_context,
                        arg2: c_int,
                        arg3: *mut *mut sqlite3_value,
                    ),
                >,
                ppArg: *mut *mut c_void,
            ) -> c_int,
        >,
        pub xRename: Option<
            unsafe extern "C" fn(
                pVtab: *mut sqlite3_vtab,
                zNew: *const c_char,
            ) -> c_int,
        >,
        pub xSavepoint: Option<
            unsafe extern "C" fn(
                pVTab: *mut sqlite3_vtab,
                arg1: c_int,
            ) -> c_int,
        >,
        pub xRelease: Option<
            unsafe extern "C" fn(
                pVTab: *mut sqlite3_vtab,
                arg1: c_int,
            ) -> c_int,
        >,
        pub xRollbackTo: Option<
            unsafe extern "C" fn(
                pVTab: *mut sqlite3_vtab,
                arg1: c_int,
            ) -> c_int,
        >,
        pub xShadowName: Option<
            unsafe extern "C" fn(arg1: *const crate::ffi::c_char) -> c_int,
        >,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_index_info {
        pub nConstraint: c_int,
        pub aConstraint: *mut sqlite3_index_constraint,
        pub nOrderBy: c_int,
        pub aOrderBy: *mut sqlite3_index_orderby,
        pub aConstraintUsage: *mut sqlite3_index_constraint_usage,
        pub idxNum: c_int,
        pub idxStr: *mut c_char,
        pub needToFreeIdxStr: c_int,
        pub orderByConsumed: c_int,
        pub estimatedCost: f64,
        pub estimatedRows: sqlite3_int64,
        pub idxFlags: c_int,
        pub colUsed: sqlite3_uint64,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_index_constraint {
        pub iColumn: c_int,
        pub op: crate::ffi::c_uchar,
        pub usable: crate::ffi::c_uchar,
        pub iTermOffset: c_int,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_index_orderby {
        pub iColumn: c_int,
        pub desc: crate::ffi::c_uchar,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_index_constraint_usage {
        pub argvIndex: c_int,
        pub omit: crate::ffi::c_uchar,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_vtab {
        pub pModule: *const sqlite3_module,
        pub nRef: c_int,
        pub zErrMsg: *mut c_char,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_vtab_cursor {
        pub pVtab: *mut sqlite3_vtab,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_blob {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_mutex_methods {
        pub xMutexInit: Option<unsafe extern "C" fn() -> c_int>,
        pub xMutexEnd: Option<unsafe extern "C" fn() -> c_int>,
        pub xMutexAlloc: Option<
            unsafe extern "C" fn(arg1: crate::ffi::c_int) -> *mut sqlite3_mutex,
        >,
        pub xMutexFree: Option<unsafe extern "C" fn(arg1: *mut sqlite3_mutex)>,
        pub xMutexEnter: Option<unsafe extern "C" fn(arg1: *mut sqlite3_mutex)>,
        pub xMutexTry: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_mutex) -> c_int,
        >,
        pub xMutexLeave: Option<unsafe extern "C" fn(arg1: *mut sqlite3_mutex)>,
        pub xMutexHeld: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_mutex) -> c_int,
        >,
        pub xMutexNotheld: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_mutex) -> c_int,
        >,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_str {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_pcache {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_pcache_page {
        pub pBuf: *mut c_void,
        pub pExtra: *mut c_void,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_pcache_methods2 {
        pub iVersion: c_int,
        pub pArg: *mut c_void,
        pub xInit: Option<
            unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
        >,
        pub xShutdown: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        pub xCreate: Option<
            unsafe extern "C" fn(
                szPage: c_int,
                szExtra: c_int,
                bPurgeable: c_int,
            ) -> *mut sqlite3_pcache,
        >,
        pub xCachesize: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_pcache, nCachesize: crate::ffi::c_int),
        >,
        pub xPagecount: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_pcache) -> c_int,
        >,
        pub xFetch: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_pcache,
                key: crate::ffi::c_uint,
                createFlag: c_int,
            ) -> *mut sqlite3_pcache_page,
        >,
        pub xUnpin: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_pcache,
                arg2: *mut sqlite3_pcache_page,
                discard: c_int,
            ),
        >,
        pub xRekey: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_pcache,
                arg2: *mut sqlite3_pcache_page,
                oldKey: crate::ffi::c_uint,
                newKey: crate::ffi::c_uint,
            ),
        >,
        pub xTruncate: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_pcache, iLimit: crate::ffi::c_uint),
        >,
        pub xDestroy: Option<unsafe extern "C" fn(arg1: *mut sqlite3_pcache)>,
        pub xShrink: Option<unsafe extern "C" fn(arg1: *mut sqlite3_pcache)>,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_pcache_methods {
        pub pArg: *mut c_void,
        pub xInit: Option<
            unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
        >,
        pub xShutdown: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        pub xCreate: Option<
            unsafe extern "C" fn(
                szPage: c_int,
                bPurgeable: c_int,
            ) -> *mut sqlite3_pcache,
        >,
        pub xCachesize: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_pcache, nCachesize: crate::ffi::c_int),
        >,
        pub xPagecount: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_pcache) -> c_int,
        >,
        pub xFetch: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_pcache,
                key: crate::ffi::c_uint,
                createFlag: c_int,
            ) -> *mut c_void,
        >,
        pub xUnpin: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_pcache,
                arg2: *mut c_void,
                discard: c_int,
            ),
        >,
        pub xRekey: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_pcache,
                arg2: *mut c_void,
                oldKey: crate::ffi::c_uint,
                newKey: crate::ffi::c_uint,
            ),
        >,
        pub xTruncate: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_pcache, iLimit: crate::ffi::c_uint),
        >,
        pub xDestroy: Option<unsafe extern "C" fn(arg1: *mut sqlite3_pcache)>,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_backup {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_snapshot {
        pub hidden: [::core::ffi::c_uchar; 48usize],
    }
    pub type sqlite3_rtree_dbl = f64;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_rtree_geometry {
        pub pContext: *mut c_void,
        pub nParam: c_int,
        pub aParam: *mut sqlite3_rtree_dbl,
        pub pUser: *mut c_void,
        pub xDelUser: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_rtree_query_info {
        pub pContext: *mut c_void,
        pub nParam: c_int,
        pub aParam: *mut sqlite3_rtree_dbl,
        pub pUser: *mut c_void,
        pub xDelUser: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        pub aCoord: *mut sqlite3_rtree_dbl,
        pub anQueue: *mut crate::ffi::c_uint,
        pub nCoord: c_int,
        pub iLevel: c_int,
        pub mxLevel: c_int,
        pub iRowid: sqlite3_int64,
        pub rParentScore: sqlite3_rtree_dbl,
        pub eParentWithin: c_int,
        pub eWithin: c_int,
        pub rScore: sqlite3_rtree_dbl,
        pub apSqlParam: *mut *mut sqlite3_value,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Fts5Context {
        _unused: [u8; 0],
    }
    pub type fts5_extension_function = Option<
        unsafe extern "C" fn(
            pApi: *const Fts5ExtensionApi,
            pFts: *mut Fts5Context,
            pCtx: *mut sqlite3_context,
            nVal: c_int,
            apVal: *mut *mut sqlite3_value,
        ),
    >;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Fts5PhraseIter {
        pub a: *const crate::ffi::c_uchar,
        pub b: *const crate::ffi::c_uchar,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Fts5ExtensionApi {
        pub iVersion: c_int,
        pub xUserData: Option<
            unsafe extern "C" fn(arg1: *mut Fts5Context) -> *mut c_void,
        >,
        pub xColumnCount:
            Option<unsafe extern "C" fn(arg1: *mut Fts5Context) -> c_int>,
        pub xRowCount: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                pnRow: *mut sqlite3_int64,
            ) -> c_int,
        >,
        pub xColumnTotalSize: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                iCol: c_int,
                pnToken: *mut sqlite3_int64,
            ) -> c_int,
        >,
        pub xTokenize: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                pText: *const c_char,
                nText: c_int,
                pCtx: *mut c_void,
                xToken: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: c_int,
                        arg3: *const c_char,
                        arg4: c_int,
                        arg5: c_int,
                        arg6: c_int,
                    ) -> c_int,
                >,
            ) -> c_int,
        >,
        pub xPhraseCount:
            Option<unsafe extern "C" fn(arg1: *mut Fts5Context) -> c_int>,
        pub xPhraseSize: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                iPhrase: c_int,
            ) -> c_int,
        >,
        pub xInstCount: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                pnInst: *mut c_int,
            ) -> c_int,
        >,
        pub xInst: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                iIdx: c_int,
                piPhrase: *mut c_int,
                piCol: *mut c_int,
                piOff: *mut c_int,
            ) -> c_int,
        >,
        pub xRowid:
            Option<unsafe extern "C" fn(arg1: *mut Fts5Context) -> sqlite3_int64>,
        pub xColumnText: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                iCol: c_int,
                pz: *mut *const c_char,
                pn: *mut c_int,
            ) -> c_int,
        >,
        pub xColumnSize: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                iCol: c_int,
                pnToken: *mut c_int,
            ) -> c_int,
        >,
        pub xQueryPhrase: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                iPhrase: c_int,
                pUserData: *mut c_void,
                arg2: Option<
                    unsafe extern "C" fn(
                        arg1: *const Fts5ExtensionApi,
                        arg2: *mut Fts5Context,
                        arg3: *mut c_void,
                    ) -> c_int,
                >,
            ) -> c_int,
        >,
        pub xSetAuxdata: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                pAux: *mut c_void,
                xDelete: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
        pub xGetAuxdata: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                bClear: c_int,
            ) -> *mut c_void,
        >,
        pub xPhraseFirst: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                iPhrase: c_int,
                arg2: *mut Fts5PhraseIter,
                arg3: *mut c_int,
                arg4: *mut c_int,
            ) -> c_int,
        >,
        pub xPhraseNext: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                arg2: *mut Fts5PhraseIter,
                piCol: *mut c_int,
                piOff: *mut c_int,
            ),
        >,
        pub xPhraseFirstColumn: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                iPhrase: c_int,
                arg2: *mut Fts5PhraseIter,
                arg3: *mut c_int,
            ) -> c_int,
        >,
        pub xPhraseNextColumn: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Context,
                arg2: *mut Fts5PhraseIter,
                piCol: *mut c_int,
            ),
        >,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Fts5Tokenizer {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct fts5_tokenizer {
        pub xCreate: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                azArg: *mut *const c_char,
                nArg: c_int,
                ppOut: *mut *mut Fts5Tokenizer,
            ) -> c_int,
        >,
        pub xDelete: Option<unsafe extern "C" fn(arg1: *mut Fts5Tokenizer)>,
        pub xTokenize: Option<
            unsafe extern "C" fn(
                arg1: *mut Fts5Tokenizer,
                pCtx: *mut c_void,
                flags: c_int,
                pText: *const c_char,
                nText: c_int,
                xToken: Option<
                    unsafe extern "C" fn(
                        pCtx: *mut c_void,
                        tflags: c_int,
                        pToken: *const c_char,
                        nToken: c_int,
                        iStart: c_int,
                        iEnd: c_int,
                    ) -> c_int,
                >,
            ) -> c_int,
        >,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct fts5_api {
        pub iVersion: c_int,
        pub xCreateTokenizer: Option<
            unsafe extern "C" fn(
                pApi: *mut fts5_api,
                zName: *const c_char,
                pContext: *mut c_void,
                pTokenizer: *mut fts5_tokenizer,
                xDestroy: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
        pub xFindTokenizer: Option<
            unsafe extern "C" fn(
                pApi: *mut fts5_api,
                zName: *const c_char,
                ppContext: *mut *mut c_void,
                pTokenizer: *mut fts5_tokenizer,
            ) -> c_int,
        >,
        pub xCreateFunction: Option<
            unsafe extern "C" fn(
                pApi: *mut fts5_api,
                zName: *const c_char,
                pContext: *mut c_void,
                xFunction: fts5_extension_function,
                xDestroy: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct sqlite3_api_routines {
        pub aggregate_context: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                nBytes: c_int,
            ) -> *mut c_void,
        >,
        pub aggregate_count: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_context) -> c_int,
        >,
        pub bind_blob: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: *const crate::ffi::c_void,
                n: c_int,
                arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
        pub bind_double: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: f64,
            ) -> c_int,
        >,
        pub bind_int: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: c_int,
            ) -> c_int,
        >,
        pub bind_int64: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: sqlite_int64,
            ) -> c_int,
        >,
        pub bind_null: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
            ) -> c_int,
        >,
        pub bind_parameter_count:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int>,
        pub bind_parameter_index: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                zName: *const c_char,
            ) -> c_int,
        >,
        pub bind_parameter_name: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
            ) -> *const c_char,
        >,
        pub bind_text: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: *const c_char,
                n: c_int,
                arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
        pub bind_text16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: *const crate::ffi::c_void,
                arg4: c_int,
                arg5: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
        pub bind_value: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: *const sqlite3_value,
            ) -> c_int,
        >,
        pub busy_handler: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: c_int,
                    ) -> c_int,
                >,
                arg3: *mut c_void,
            ) -> c_int,
        >,
        pub busy_timeout: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3, ms: crate::ffi::c_int) -> c_int,
        >,
        pub changes:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int>,
        pub close:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int>,
        pub collation_needed: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *mut c_void,
                arg3: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: *mut sqlite3,
                        eTextRep: c_int,
                        arg3: *const c_char,
                    ),
                >,
            ) -> c_int,
        >,
        pub collation_needed16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *mut c_void,
                arg3: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: *mut sqlite3,
                        eTextRep: c_int,
                        arg3: *const crate::ffi::c_void,
                    ),
                >,
            ) -> c_int,
        >,
        pub column_blob: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                iCol: c_int,
            ) -> *const crate::ffi::c_void,
        >,
        pub column_bytes: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                iCol: c_int,
            ) -> c_int,
        >,
        pub column_bytes16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                iCol: c_int,
            ) -> c_int,
        >,
        pub column_count: Option<
            unsafe extern "C" fn(pStmt: *mut sqlite3_stmt) -> c_int,
        >,
        pub column_database_name: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
            ) -> *const c_char,
        >,
        pub column_database_name16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
            ) -> *const crate::ffi::c_void,
        >,
        pub column_decltype: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                i: c_int,
            ) -> *const c_char,
        >,
        pub column_decltype16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
            ) -> *const crate::ffi::c_void,
        >,
        pub column_double: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_stmt, iCol: crate::ffi::c_int) -> f64,
        >,
        pub column_int: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                iCol: c_int,
            ) -> c_int,
        >,
        pub column_int64: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_stmt, iCol: crate::ffi::c_int) -> sqlite_int64,
        >,
        pub column_name: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
            ) -> *const c_char,
        >,
        pub column_name16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
            ) -> *const crate::ffi::c_void,
        >,
        pub column_origin_name: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
            ) -> *const c_char,
        >,
        pub column_origin_name16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
            ) -> *const crate::ffi::c_void,
        >,
        pub column_table_name: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
            ) -> *const c_char,
        >,
        pub column_table_name16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
            ) -> *const crate::ffi::c_void,
        >,
        pub column_text: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                iCol: c_int,
            ) -> *const crate::ffi::c_uchar,
        >,
        pub column_text16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                iCol: c_int,
            ) -> *const crate::ffi::c_void,
        >,
        pub column_type: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                iCol: c_int,
            ) -> c_int,
        >,
        pub column_value: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                iCol: c_int,
            ) -> *mut sqlite3_value,
        >,
        pub commit_hook: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: Option<
                    unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
                >,
                arg3: *mut c_void,
            ) -> *mut c_void,
        >,
        pub complete: Option<
            unsafe extern "C" fn(sql: *const crate::ffi::c_char) -> c_int,
        >,
        pub complete16: Option<
            unsafe extern "C" fn(sql: *const crate::ffi::c_void) -> c_int,
        >,
        pub create_collation: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: c_int,
                arg4: *mut c_void,
                arg5: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: c_int,
                        arg3: *const crate::ffi::c_void,
                        arg4: c_int,
                        arg5: *const crate::ffi::c_void,
                    ) -> c_int,
                >,
            ) -> c_int,
        >,
        pub create_collation16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const crate::ffi::c_void,
                arg3: c_int,
                arg4: *mut c_void,
                arg5: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: c_int,
                        arg3: *const crate::ffi::c_void,
                        arg4: c_int,
                        arg5: *const crate::ffi::c_void,
                    ) -> c_int,
                >,
            ) -> c_int,
        >,
        pub create_function: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: c_int,
                arg4: c_int,
                arg5: *mut c_void,
                xFunc: Option<
                    unsafe extern "C" fn(
                        arg1: *mut sqlite3_context,
                        arg2: c_int,
                        arg3: *mut *mut sqlite3_value,
                    ),
                >,
                xStep: Option<
                    unsafe extern "C" fn(
                        arg1: *mut sqlite3_context,
                        arg2: c_int,
                        arg3: *mut *mut sqlite3_value,
                    ),
                >,
                xFinal: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
            ) -> c_int,
        >,
        pub create_function16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const crate::ffi::c_void,
                arg3: c_int,
                arg4: c_int,
                arg5: *mut c_void,
                xFunc: Option<
                    unsafe extern "C" fn(
                        arg1: *mut sqlite3_context,
                        arg2: c_int,
                        arg3: *mut *mut sqlite3_value,
                    ),
                >,
                xStep: Option<
                    unsafe extern "C" fn(
                        arg1: *mut sqlite3_context,
                        arg2: c_int,
                        arg3: *mut *mut sqlite3_value,
                    ),
                >,
                xFinal: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
            ) -> c_int,
        >,
        pub create_module: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: *const sqlite3_module,
                arg4: *mut c_void,
            ) -> c_int,
        >,
        pub data_count: Option<
            unsafe extern "C" fn(pStmt: *mut sqlite3_stmt) -> c_int,
        >,
        pub db_handle:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> *mut sqlite3>,
        pub declare_vtab: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
            ) -> c_int,
        >,
        pub enable_shared_cache: Option<
            unsafe extern "C" fn(arg1: crate::ffi::c_int) -> c_int,
        >,
        pub errcode:
            Option<unsafe extern "C" fn(db: *mut sqlite3) -> c_int>,
        pub errmsg: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3) -> *const c_char,
        >,
        pub errmsg16: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3) -> *const crate::ffi::c_void,
        >,
        pub exec: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: sqlite3_callback,
                arg4: *mut c_void,
                arg5: *mut *mut c_char,
            ) -> c_int,
        >,
        pub expired:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int>,
        pub finalize: Option<
            unsafe extern "C" fn(pStmt: *mut sqlite3_stmt) -> c_int,
        >,
        pub free: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        pub free_table:
            Option<unsafe extern "C" fn(result: *mut *mut crate::ffi::c_char)>,
        pub get_autocommit:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int>,
        pub get_auxdata: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: c_int,
            ) -> *mut c_void,
        >,
        pub get_table: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: *mut *mut *mut c_char,
                arg4: *mut c_int,
                arg5: *mut c_int,
                arg6: *mut *mut c_char,
            ) -> c_int,
        >,
        pub global_recover: Option<unsafe extern "C" fn() -> c_int>,
        pub interruptx: Option<unsafe extern "C" fn(arg1: *mut sqlite3)>,
        pub last_insert_rowid:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> sqlite_int64>,
        pub libversion: Option<unsafe extern "C" fn() -> *const crate::ffi::c_char>,
        pub libversion_number: Option<unsafe extern "C" fn() -> c_int>,
        pub malloc: Option<
            unsafe extern "C" fn(arg1: crate::ffi::c_int) -> *mut c_void,
        >,
        pub mprintf: Option<
            unsafe extern "C" fn(arg1: *const c_char, ...) -> *mut c_char,
        >,
        pub open: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: *mut *mut sqlite3,
            ) -> c_int,
        >,
        pub open16: Option<
            unsafe extern "C" fn(
                arg1: *const crate::ffi::c_void,
                arg2: *mut *mut sqlite3,
            ) -> c_int,
        >,
        pub prepare: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: c_int,
                arg4: *mut *mut sqlite3_stmt,
                arg5: *mut *const c_char,
            ) -> c_int,
        >,
        pub prepare16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const crate::ffi::c_void,
                arg3: c_int,
                arg4: *mut *mut sqlite3_stmt,
                arg5: *mut *const crate::ffi::c_void,
            ) -> c_int,
        >,
        pub profile: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: *const c_char,
                        arg3: sqlite_uint64,
                    ),
                >,
                arg3: *mut c_void,
            ) -> *mut c_void,
        >,
        pub progress_handler: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: c_int,
                arg3: Option<
                    unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
                >,
                arg4: *mut c_void,
            ),
        >,
        pub realloc: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: c_int,
            ) -> *mut c_void,
        >,
        pub reset: Option<
            unsafe extern "C" fn(pStmt: *mut sqlite3_stmt) -> c_int,
        >,
        pub result_blob: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: *const crate::ffi::c_void,
                arg3: c_int,
                arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ),
        >,
        pub result_double:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_context, arg2: f64)>,
        pub result_error: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: *const c_char,
                arg3: c_int,
            ),
        >,
        pub result_error16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: *const crate::ffi::c_void,
                arg3: c_int,
            ),
        >,
        pub result_int: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_context, arg2: crate::ffi::c_int),
        >,
        pub result_int64: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_context, arg2: sqlite_int64),
        >,
        pub result_null: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
        pub result_text: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: *const c_char,
                arg3: c_int,
                arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ),
        >,
        pub result_text16: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: *const crate::ffi::c_void,
                arg3: c_int,
                arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ),
        >,
        pub result_text16be: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: *const crate::ffi::c_void,
                arg3: c_int,
                arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ),
        >,
        pub result_text16le: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: *const crate::ffi::c_void,
                arg3: c_int,
                arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ),
        >,
        pub result_value: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_context, arg2: *mut sqlite3_value),
        >,
        pub rollback_hook: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
                arg3: *mut c_void,
            ) -> *mut c_void,
        >,
        pub set_authorizer: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: c_int,
                        arg3: *const c_char,
                        arg4: *const c_char,
                        arg5: *const c_char,
                        arg6: *const c_char,
                    ) -> c_int,
                >,
                arg3: *mut c_void,
            ) -> c_int,
        >,
        pub set_auxdata: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: c_int,
                arg3: *mut c_void,
                arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ),
        >,
        pub xsnprintf: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: *mut c_char,
                arg3: *const c_char,
                ...
            ) -> *mut c_char,
        >,
        pub step:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int>,
        pub table_column_metadata: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: *const c_char,
                arg4: *const c_char,
                arg5: *mut *const c_char,
                arg6: *mut *const c_char,
                arg7: *mut c_int,
                arg8: *mut c_int,
                arg9: *mut c_int,
            ) -> c_int,
        >,
        pub thread_cleanup: Option<unsafe extern "C" fn()>,
        pub total_changes:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int>,
        pub trace: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                xTrace: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: *const c_char,
                    ),
                >,
                arg2: *mut c_void,
            ) -> *mut c_void,
        >,
        pub transfer_bindings: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: *mut sqlite3_stmt,
            ) -> c_int,
        >,
        pub update_hook: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: c_int,
                        arg3: *const c_char,
                        arg4: *const c_char,
                        arg5: sqlite_int64,
                    ),
                >,
                arg3: *mut c_void,
            ) -> *mut c_void,
        >,
        pub user_data: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_context) -> *mut c_void,
        >,
        pub value_blob: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> *const crate::ffi::c_void,
        >,
        pub value_bytes: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int,
        >,
        pub value_bytes16: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int,
        >,
        pub value_double: Option<unsafe extern "C" fn(arg1: *mut sqlite3_value) -> f64>,
        pub value_int: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int,
        >,
        pub value_int64:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_value) -> sqlite_int64>,
        pub value_numeric_type: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int,
        >,
        pub value_text: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> *const crate::ffi::c_uchar,
        >,
        pub value_text16: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> *const crate::ffi::c_void,
        >,
        pub value_text16be: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> *const crate::ffi::c_void,
        >,
        pub value_text16le: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> *const crate::ffi::c_void,
        >,
        pub value_type: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int,
        >,
        pub vmprintf: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: *mut c_void,
            ) -> *mut c_char,
        >,
        pub overload_function: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                zFuncName: *const c_char,
                nArg: c_int,
            ) -> c_int,
        >,
        pub prepare_v2: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: c_int,
                arg4: *mut *mut sqlite3_stmt,
                arg5: *mut *const c_char,
            ) -> c_int,
        >,
        pub prepare16_v2: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const crate::ffi::c_void,
                arg3: c_int,
                arg4: *mut *mut sqlite3_stmt,
                arg5: *mut *const crate::ffi::c_void,
            ) -> c_int,
        >,
        pub clear_bindings:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int>,
        pub create_module_v2: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: *const sqlite3_module,
                arg4: *mut c_void,
                xDestroy: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
        pub bind_zeroblob: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: c_int,
            ) -> c_int,
        >,
        pub blob_bytes:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_blob) -> c_int>,
        pub blob_close:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_blob) -> c_int>,
        pub blob_open: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: *const c_char,
                arg4: *const c_char,
                arg5: sqlite3_int64,
                arg6: c_int,
                arg7: *mut *mut sqlite3_blob,
            ) -> c_int,
        >,
        pub blob_read: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_blob,
                arg2: *mut c_void,
                arg3: c_int,
                arg4: c_int,
            ) -> c_int,
        >,
        pub blob_write: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_blob,
                arg2: *const crate::ffi::c_void,
                arg3: c_int,
                arg4: c_int,
            ) -> c_int,
        >,
        pub create_collation_v2: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: c_int,
                arg4: *mut c_void,
                arg5: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: c_int,
                        arg3: *const crate::ffi::c_void,
                        arg4: c_int,
                        arg5: *const crate::ffi::c_void,
                    ) -> c_int,
                >,
                arg6: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
        pub file_control: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: c_int,
                arg4: *mut c_void,
            ) -> c_int,
        >,
        pub memory_highwater:
            Option<unsafe extern "C" fn(arg1: crate::ffi::c_int) -> sqlite3_int64>,
        pub memory_used: Option<unsafe extern "C" fn() -> sqlite3_int64>,
        pub mutex_alloc: Option<
            unsafe extern "C" fn(arg1: crate::ffi::c_int) -> *mut sqlite3_mutex,
        >,
        pub mutex_enter: Option<unsafe extern "C" fn(arg1: *mut sqlite3_mutex)>,
        pub mutex_free: Option<unsafe extern "C" fn(arg1: *mut sqlite3_mutex)>,
        pub mutex_leave: Option<unsafe extern "C" fn(arg1: *mut sqlite3_mutex)>,
        pub mutex_try: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_mutex) -> c_int,
        >,
        pub open_v2: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: *mut *mut sqlite3,
                arg3: c_int,
                arg4: *const c_char,
            ) -> c_int,
        >,
        pub release_memory: Option<
            unsafe extern "C" fn(arg1: crate::ffi::c_int) -> c_int,
        >,
        pub result_error_nomem:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
        pub result_error_toobig:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
        pub sleep: Option<
            unsafe extern "C" fn(arg1: crate::ffi::c_int) -> c_int,
        >,
        pub soft_heap_limit: Option<unsafe extern "C" fn(arg1: crate::ffi::c_int)>,
        pub vfs_find: Option<
            unsafe extern "C" fn(arg1: *const crate::ffi::c_char) -> *mut sqlite3_vfs,
        >,
        pub vfs_register: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_vfs,
                arg2: c_int,
            ) -> c_int,
        >,
        pub vfs_unregister:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_vfs) -> c_int>,
        pub xthreadsafe: Option<unsafe extern "C" fn() -> c_int>,
        pub result_zeroblob: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_context, arg2: crate::ffi::c_int),
        >,
        pub result_error_code: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_context, arg2: crate::ffi::c_int),
        >,
        pub test_control: Option<
            unsafe extern "C" fn(arg1: c_int, ...) -> c_int,
        >,
        pub randomness: Option<
            unsafe extern "C" fn(arg1: c_int, arg2: *mut c_void),
        >,
        pub context_db_handle:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_context) -> *mut sqlite3>,
        pub extended_result_codes: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3, arg2: crate::ffi::c_int) -> c_int,
        >,
        pub limit: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: c_int,
                arg3: c_int,
            ) -> c_int,
        >,
        pub next_stmt: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3, arg2: *mut sqlite3_stmt) -> *mut sqlite3_stmt,
        >,
        pub sql: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> *const c_char,
        >,
        pub status: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: *mut c_int,
                arg3: *mut c_int,
                arg4: c_int,
            ) -> c_int,
        >,
        pub backup_finish: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_backup) -> c_int,
        >,
        pub backup_init: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: *mut sqlite3,
                arg4: *const c_char,
            ) -> *mut sqlite3_backup,
        >,
        pub backup_pagecount: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_backup) -> c_int,
        >,
        pub backup_remaining: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_backup) -> c_int,
        >,
        pub backup_step: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_backup,
                arg2: c_int,
            ) -> c_int,
        >,
        pub compileoption_get: Option<
            unsafe extern "C" fn(arg1: crate::ffi::c_int) -> *const c_char,
        >,
        pub compileoption_used: Option<
            unsafe extern "C" fn(arg1: *const crate::ffi::c_char) -> c_int,
        >,
        pub create_function_v2: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: c_int,
                arg4: c_int,
                arg5: *mut c_void,
                xFunc: Option<
                    unsafe extern "C" fn(
                        arg1: *mut sqlite3_context,
                        arg2: c_int,
                        arg3: *mut *mut sqlite3_value,
                    ),
                >,
                xStep: Option<
                    unsafe extern "C" fn(
                        arg1: *mut sqlite3_context,
                        arg2: c_int,
                        arg3: *mut *mut sqlite3_value,
                    ),
                >,
                xFinal: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
                xDestroy: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
        pub db_config: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: c_int,
                ...
            ) -> c_int,
        >,
        pub db_mutex:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> *mut sqlite3_mutex>,
        pub db_status: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: c_int,
                arg3: *mut c_int,
                arg4: *mut c_int,
                arg5: c_int,
            ) -> c_int,
        >,
        pub extended_errcode:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int>,
        pub log: Option<
            unsafe extern "C" fn(arg1: c_int, arg2: *const c_char, ...),
        >,
        pub soft_heap_limit64:
            Option<unsafe extern "C" fn(arg1: sqlite3_int64) -> sqlite3_int64>,
        pub sourceid: Option<unsafe extern "C" fn() -> *const crate::ffi::c_char>,
        pub stmt_status: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: c_int,
            ) -> c_int,
        >,
        pub strnicmp: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: *const c_char,
                arg3: c_int,
            ) -> c_int,
        >,
        pub unlock_notify: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: Option<
                    unsafe extern "C" fn(arg1: *mut *mut c_void, arg2: crate::ffi::c_int),
                >,
                arg3: *mut c_void,
            ) -> c_int,
        >,
        pub wal_autocheckpoint: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3, arg2: crate::ffi::c_int) -> c_int,
        >,
        pub wal_checkpoint: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
            ) -> c_int,
        >,
        pub wal_hook: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: Option<
                    unsafe extern "C" fn(
                        arg1: *mut c_void,
                        arg2: *mut sqlite3,
                        arg3: *const c_char,
                        arg4: c_int,
                    ) -> c_int,
                >,
                arg3: *mut c_void,
            ) -> *mut c_void,
        >,
        pub blob_reopen: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_blob, arg2: sqlite3_int64) -> c_int,
        >,
        pub vtab_config: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3, op: c_int, ...) -> c_int,
        >,
        pub vtab_on_conflict:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int>,
        pub close_v2:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int>,
        pub db_filename: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
            ) -> *const c_char,
        >,
        pub db_readonly: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
            ) -> c_int,
        >,
        pub db_release_memory:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int>,
        pub errstr: Option<
            unsafe extern "C" fn(arg1: crate::ffi::c_int) -> *const c_char,
        >,
        pub stmt_busy:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int>,
        pub stmt_readonly:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int>,
        pub stricmp: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: *const c_char,
            ) -> c_int,
        >,
        pub uri_boolean: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: *const c_char,
                arg3: c_int,
            ) -> c_int,
        >,
        pub uri_int64: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: *const c_char,
                arg3: sqlite3_int64,
            ) -> sqlite3_int64,
        >,
        pub uri_parameter: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: *const c_char,
            ) -> *const c_char,
        >,
        pub xvsnprintf: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: *mut c_char,
                arg3: *const c_char,
                arg4: *mut c_void,
            ) -> *mut c_char,
        >,
        pub wal_checkpoint_v2: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: c_int,
                arg4: *mut c_int,
                arg5: *mut c_int,
            ) -> c_int,
        >,
        pub auto_extension: Option<
            unsafe extern "C" fn(
                arg1: Option<unsafe extern "C" fn()>,
            ) -> c_int,
        >,
        pub bind_blob64: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: *const crate::ffi::c_void,
                arg4: sqlite3_uint64,
                arg5: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
        pub bind_text64: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: *const c_char,
                arg4: sqlite3_uint64,
                arg5: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
                arg6: crate::ffi::c_uchar,
            ) -> c_int,
        >,
        pub cancel_auto_extension: Option<
            unsafe extern "C" fn(
                arg1: Option<unsafe extern "C" fn()>,
            ) -> c_int,
        >,
        pub load_extension: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: *const c_char,
                arg4: *mut *mut c_char,
            ) -> c_int,
        >,
        pub malloc64: Option<
            unsafe extern "C" fn(arg1: sqlite3_uint64) -> *mut c_void,
        >,
        pub msize: Option<
            unsafe extern "C" fn(arg1: *mut c_void) -> sqlite3_uint64,
        >,
        pub realloc64: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: sqlite3_uint64,
            ) -> *mut c_void,
        >,
        pub reset_auto_extension: Option<unsafe extern "C" fn()>,
        pub result_blob64: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: *const crate::ffi::c_void,
                arg3: sqlite3_uint64,
                arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ),
        >,
        pub result_text64: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: *const c_char,
                arg3: sqlite3_uint64,
                arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
                arg5: crate::ffi::c_uchar,
            ),
        >,
        pub strglob: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: *const c_char,
            ) -> c_int,
        >,
        pub value_dup: Option<
            unsafe extern "C" fn(arg1: *const sqlite3_value) -> *mut sqlite3_value,
        >,
        pub value_free: Option<unsafe extern "C" fn(arg1: *mut sqlite3_value)>,
        pub result_zeroblob64: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: sqlite3_uint64,
            ) -> c_int,
        >,
        pub bind_zeroblob64: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: sqlite3_uint64,
            ) -> c_int,
        >,
        pub value_subtype: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> ::core::ffi::c_uint,
        >,
        pub result_subtype: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_context, arg2: crate::ffi::c_uint),
        >,
        pub status64: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: *mut sqlite3_int64,
                arg3: *mut sqlite3_int64,
                arg4: c_int,
            ) -> c_int,
        >,
        pub strlike: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: *const c_char,
                arg3: crate::ffi::c_uint,
            ) -> c_int,
        >,
        pub db_cacheflush:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int>,
        pub system_errno:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int>,
        pub trace_v2: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: crate::ffi::c_uint,
                arg3: Option<
                    unsafe extern "C" fn(
                        arg1: crate::ffi::c_uint,
                        arg2: *mut c_void,
                        arg3: *mut c_void,
                        arg4: *mut c_void,
                    ) -> c_int,
                >,
                arg4: *mut c_void,
            ) -> c_int,
        >,
        pub expanded_sql: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> *mut c_char,
        >,
        pub set_last_insert_rowid:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3, arg2: sqlite3_int64)>,
        pub prepare_v3: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: c_int,
                arg4: crate::ffi::c_uint,
                arg5: *mut *mut sqlite3_stmt,
                arg6: *mut *const c_char,
            ) -> c_int,
        >,
        pub prepare16_v3: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const crate::ffi::c_void,
                arg3: c_int,
                arg4: crate::ffi::c_uint,
                arg5: *mut *mut sqlite3_stmt,
                arg6: *mut *const crate::ffi::c_void,
            ) -> c_int,
        >,
        pub bind_pointer: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_stmt,
                arg2: c_int,
                arg3: *mut c_void,
                arg4: *const c_char,
                arg5: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
        pub result_pointer: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: *mut c_void,
                arg3: *const c_char,
                arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ),
        >,
        pub value_pointer: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_value,
                arg2: *const c_char,
            ) -> *mut c_void,
        >,
        pub vtab_nochange: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_context) -> c_int,
        >,
        pub value_nochange: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int,
        >,
        pub vtab_collation: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_index_info,
                arg2: c_int,
            ) -> *const c_char,
        >,
        pub keyword_count: Option<unsafe extern "C" fn() -> c_int>,
        pub keyword_name: Option<
            unsafe extern "C" fn(
                arg1: c_int,
                arg2: *mut *const c_char,
                arg3: *mut c_int,
            ) -> c_int,
        >,
        pub keyword_check: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: c_int,
            ) -> c_int,
        >,
        pub str_new:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3) -> *mut sqlite3_str>,
        pub str_finish: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_str) -> *mut c_char,
        >,
        pub str_appendf: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_str, zFormat: *const c_char, ...),
        >,
        pub str_vappendf: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_str,
                zFormat: *const c_char,
                arg2: *mut c_void,
            ),
        >,
        pub str_append: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_str,
                zIn: *const c_char,
                N: c_int,
            ),
        >,
        pub str_appendall: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_str, zIn: *const crate::ffi::c_char),
        >,
        pub str_appendchar: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_str, N: c_int, C: crate::ffi::c_char),
        >,
        pub str_reset: Option<unsafe extern "C" fn(arg1: *mut sqlite3_str)>,
        pub str_errcode:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_str) -> c_int>,
        pub str_length:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_str) -> c_int>,
        pub str_value: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_str) -> *mut c_char,
        >,
        pub create_window_function: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
                arg3: c_int,
                arg4: c_int,
                arg5: *mut c_void,
                xStep: Option<
                    unsafe extern "C" fn(
                        arg1: *mut sqlite3_context,
                        arg2: c_int,
                        arg3: *mut *mut sqlite3_value,
                    ),
                >,
                xFinal: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
                xValue: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
                xInv: Option<
                    unsafe extern "C" fn(
                        arg1: *mut sqlite3_context,
                        arg2: c_int,
                        arg3: *mut *mut sqlite3_value,
                    ),
                >,
                xDestroy: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            ) -> c_int,
        >,
        pub normalized_sql: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> *const c_char,
        >,
        pub stmt_isexplain:
            Option<unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int>,
        pub value_frombind: Option<
            unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int,
        >,
        pub drop_modules: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *mut *const c_char,
            ) -> c_int,
        >,
        pub hard_heap_limit64:
            Option<unsafe extern "C" fn(arg1: sqlite3_int64) -> sqlite3_int64>,
        pub uri_key: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: c_int,
            ) -> *const c_char,
        >,
        pub filename_database: Option<
            unsafe extern "C" fn(arg1: *const crate::ffi::c_char) -> *const c_char,
        >,
        pub filename_journal: Option<
            unsafe extern "C" fn(arg1: *const crate::ffi::c_char) -> *const c_char,
        >,
        pub filename_wal: Option<
            unsafe extern "C" fn(arg1: *const crate::ffi::c_char) -> *const c_char,
        >,
        pub create_filename: Option<
            unsafe extern "C" fn(
                arg1: *const c_char,
                arg2: *const c_char,
                arg3: *const c_char,
                arg4: c_int,
                arg5: *mut *const c_char,
            ) -> *mut c_char,
        >,
        pub free_filename: Option<unsafe extern "C" fn(arg1: *mut crate::ffi::c_char)>,
        pub database_file_object: Option<
            unsafe extern "C" fn(arg1: *const crate::ffi::c_char) -> *mut sqlite3_file,
        >,
        pub txn_state: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3,
                arg2: *const c_char,
            ) -> c_int,
        >,
    }
    pub type sqlite3_loadext_entry = Option<
        unsafe extern "C" fn(
            db: *mut sqlite3,
            pzErrMsg: *mut *mut c_char,
            pThunk: *const sqlite3_api_routines,
        ) -> c_int,
    >;
    static __SQLITE3_AGGREGATE_CONTEXT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_aggregate_context(
        arg1: *mut sqlite3_context,
        nBytes: c_int,
    ) -> *mut c_void {
        let ptr = __SQLITE3_AGGREGATE_CONTEXT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            nBytes: c_int,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, nBytes)
    }

    static __SQLITE3_BIND_BLOB: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_blob(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: *const crate::ffi::c_void,
        n: c_int,
        arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_BLOB.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: *const crate::ffi::c_void,
            n: c_int,
            arg4: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, n, arg4)
    }

    static __SQLITE3_BIND_DOUBLE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_double(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: f64,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_DOUBLE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: f64,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_BIND_INT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_int(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_INT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_BIND_INT64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_int64(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: sqlite_int64,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_INT64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: sqlite_int64,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_BIND_NULL: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_null(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_NULL.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_BIND_PARAMETER_COUNT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_parameter_count(
        arg1: *mut sqlite3_stmt,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_PARAMETER_COUNT
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_BIND_PARAMETER_INDEX: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_parameter_index(
        arg1: *mut sqlite3_stmt,
        zName: *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_PARAMETER_INDEX
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            zName: *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, zName)
    }

    static __SQLITE3_BIND_PARAMETER_NAME: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_parameter_name(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_BIND_PARAMETER_NAME
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_BIND_TEXT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_text(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: *const c_char,
        n: c_int,
        arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_TEXT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: *const c_char,
            n: c_int,
            arg4: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, n, arg4)
    }

    static __SQLITE3_BIND_VALUE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_value(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: *const sqlite3_value,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_VALUE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: *const sqlite3_value,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_BUSY_HANDLER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_busy_handler(
        arg1: *mut sqlite3,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: c_int,
            ) -> c_int,
        >,
        arg3: *mut c_void,
    ) -> c_int {
        let ptr = __SQLITE3_BUSY_HANDLER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: Option<
                unsafe extern "C" fn(
                    arg1: *mut c_void,
                    arg2: c_int,
                ) -> c_int,
            >,
            arg3: *mut c_void,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_BUSY_TIMEOUT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_busy_timeout(
        arg1: *mut sqlite3,
        ms: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_BUSY_TIMEOUT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            ms: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, ms)
    }

    static __SQLITE3_CHANGES: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_changes(arg1: *mut sqlite3) -> c_int {
        let ptr = __SQLITE3_CHANGES.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_CLOSE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_close(arg1: *mut sqlite3) -> c_int {
        let ptr = __SQLITE3_CLOSE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_COLLATION_NEEDED: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_collation_needed(
        arg1: *mut sqlite3,
        arg2: *mut c_void,
        arg3: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: *mut sqlite3,
                eTextRep: c_int,
                arg3: *const c_char,
            ),
        >,
    ) -> c_int {
        let ptr = __SQLITE3_COLLATION_NEEDED.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *mut c_void,
            arg3: Option<
                unsafe extern "C" fn(
                    arg1: *mut c_void,
                    arg2: *mut sqlite3,
                    eTextRep: c_int,
                    arg3: *const c_char,
                ),
            >,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_COLUMN_BLOB: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_blob(
        arg1: *mut sqlite3_stmt,
        iCol: c_int,
    ) -> *const crate::ffi::c_void {
        let ptr = __SQLITE3_COLUMN_BLOB.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            iCol: c_int,
        ) -> *const crate::ffi::c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, iCol)
    }

    static __SQLITE3_COLUMN_BYTES: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_bytes(
        arg1: *mut sqlite3_stmt,
        iCol: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_COLUMN_BYTES.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            iCol: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, iCol)
    }

    static __SQLITE3_COLUMN_COUNT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> c_int {
        let ptr = __SQLITE3_COLUMN_COUNT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(pStmt: *mut sqlite3_stmt) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(pStmt)
    }

    static __SQLITE3_COLUMN_DATABASE_NAME: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_database_name(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_COLUMN_DATABASE_NAME
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_COLUMN_DECLTYPE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_decltype(
        arg1: *mut sqlite3_stmt,
        i: c_int,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_COLUMN_DECLTYPE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            i: c_int,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1, i)
    }

    static __SQLITE3_COLUMN_DOUBLE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_double(
        arg1: *mut sqlite3_stmt,
        iCol: c_int,
    ) -> f64 {
        let ptr = __SQLITE3_COLUMN_DOUBLE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            iCol: c_int,
        ) -> f64 = ::core::mem::transmute(ptr);
        (fun)(arg1, iCol)
    }

    static __SQLITE3_COLUMN_INT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_int(
        arg1: *mut sqlite3_stmt,
        iCol: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_COLUMN_INT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            iCol: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, iCol)
    }

    static __SQLITE3_COLUMN_INT64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_int64(
        arg1: *mut sqlite3_stmt,
        iCol: c_int,
    ) -> sqlite_int64 {
        let ptr = __SQLITE3_COLUMN_INT64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            iCol: c_int,
        ) -> sqlite_int64 = ::core::mem::transmute(ptr);
        (fun)(arg1, iCol)
    }

    static __SQLITE3_COLUMN_NAME: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_name(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_COLUMN_NAME.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_COLUMN_ORIGIN_NAME: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_origin_name(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_COLUMN_ORIGIN_NAME.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_COLUMN_TABLE_NAME: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_table_name(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_COLUMN_TABLE_NAME.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_COLUMN_TEXT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_text(
        arg1: *mut sqlite3_stmt,
        iCol: c_int,
    ) -> *const crate::ffi::c_uchar {
        let ptr = __SQLITE3_COLUMN_TEXT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            iCol: c_int,
        ) -> *const crate::ffi::c_uchar = ::core::mem::transmute(ptr);
        (fun)(arg1, iCol)
    }

    static __SQLITE3_COLUMN_TYPE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_type(
        arg1: *mut sqlite3_stmt,
        iCol: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_COLUMN_TYPE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            iCol: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, iCol)
    }

    static __SQLITE3_COLUMN_VALUE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_column_value(
        arg1: *mut sqlite3_stmt,
        iCol: c_int,
    ) -> *mut sqlite3_value {
        let ptr = __SQLITE3_COLUMN_VALUE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            iCol: c_int,
        ) -> *mut sqlite3_value = ::core::mem::transmute(ptr);
        (fun)(arg1, iCol)
    }

    static __SQLITE3_COMMIT_HOOK: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_commit_hook(
        arg1: *mut sqlite3,
        arg2: Option<
            unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
        >,
        arg3: *mut c_void,
    ) -> *mut c_void {
        let ptr = __SQLITE3_COMMIT_HOOK.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: Option<
                unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
            >,
            arg3: *mut c_void,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_COMPLETE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_complete(sql: *const crate::ffi::c_char) -> c_int {
        let ptr = __SQLITE3_COMPLETE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            sql: *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(sql)
    }

    static __SQLITE3_DATA_COUNT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_data_count(pStmt: *mut sqlite3_stmt) -> c_int {
        let ptr = __SQLITE3_DATA_COUNT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(pStmt: *mut sqlite3_stmt) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(pStmt)
    }

    static __SQLITE3_DB_HANDLE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_db_handle(arg1: *mut sqlite3_stmt) -> *mut sqlite3 {
        let ptr = __SQLITE3_DB_HANDLE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> *mut sqlite3 = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_DECLARE_VTAB: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_declare_vtab(
        arg1: *mut sqlite3,
        arg2: *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_DECLARE_VTAB.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_ENABLE_SHARED_CACHE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_enable_shared_cache(
        arg1: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_ENABLE_SHARED_CACHE
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: crate::ffi::c_int) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_ERRCODE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_errcode(db: *mut sqlite3) -> c_int {
        let ptr = __SQLITE3_ERRCODE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(db: *mut sqlite3) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(db)
    }

    static __SQLITE3_ERRMSG: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_errmsg(arg1: *mut sqlite3) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_ERRMSG.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> *const crate::ffi::c_char = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_EXEC: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_exec(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: sqlite3_callback,
        arg4: *mut c_void,
        arg5: *mut *mut c_char,
    ) -> c_int {
        let ptr = __SQLITE3_EXEC.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: sqlite3_callback,
            arg4: *mut c_void,
            arg5: *mut *mut c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5)
    }

    static __SQLITE3_FINALIZE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> c_int {
        let ptr = __SQLITE3_FINALIZE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(pStmt: *mut sqlite3_stmt) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(pStmt)
    }

    static __SQLITE3_FREE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_free(arg1: *mut c_void) {
        let ptr = __SQLITE3_FREE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut c_void) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_FREE_TABLE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_free_table(result: *mut *mut crate::ffi::c_char) {
        let ptr = __SQLITE3_FREE_TABLE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(result: *mut *mut crate::ffi::c_char) = ::core::mem::transmute(
            ptr,
        );
        (fun)(result)
    }

    static __SQLITE3_GET_AUTOCOMMIT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_get_autocommit(arg1: *mut sqlite3) -> c_int {
        let ptr = __SQLITE3_GET_AUTOCOMMIT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_GET_AUXDATA: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_get_auxdata(
        arg1: *mut sqlite3_context,
        arg2: c_int,
    ) -> *mut c_void {
        let ptr = __SQLITE3_GET_AUXDATA.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: c_int,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_GET_TABLE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_get_table(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: *mut *mut *mut c_char,
        arg4: *mut c_int,
        arg5: *mut c_int,
        arg6: *mut *mut c_char,
    ) -> c_int {
        let ptr = __SQLITE3_GET_TABLE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: *mut *mut *mut c_char,
            arg4: *mut c_int,
            arg5: *mut c_int,
            arg6: *mut *mut c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5, arg6)
    }

    static __SQLITE3_INTERRUPT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_interrupt(arg1: *mut sqlite3) {
        let ptr = __SQLITE3_INTERRUPT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_LAST_INSERT_ROWID: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_last_insert_rowid(arg1: *mut sqlite3) -> sqlite_int64 {
        let ptr = __SQLITE3_LAST_INSERT_ROWID.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> sqlite_int64 = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_LIBVERSION: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_libversion() -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_LIBVERSION.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn() -> *const crate::ffi::c_char = ::core::mem::transmute(
            ptr,
        );
        (fun)()
    }

    static __SQLITE3_LIBVERSION_NUMBER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_libversion_number() -> c_int {
        let ptr = __SQLITE3_LIBVERSION_NUMBER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn() -> c_int = ::core::mem::transmute(ptr);
        (fun)()
    }

    static __SQLITE3_MALLOC: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_malloc(arg1: crate::ffi::c_int) -> *mut c_void {
        let ptr = __SQLITE3_MALLOC.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: c_int,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_OPEN: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_open(
        arg1: *const c_char,
        arg2: *mut *mut sqlite3,
    ) -> c_int {
        let ptr = __SQLITE3_OPEN.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *mut *mut sqlite3,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_PROFILE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_profile(
        arg1: *mut sqlite3,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: *const c_char,
                arg3: sqlite_uint64,
            ),
        >,
        arg3: *mut c_void,
    ) -> *mut c_void {
        let ptr = __SQLITE3_PROFILE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: Option<
                unsafe extern "C" fn(
                    arg1: *mut c_void,
                    arg2: *const c_char,
                    arg3: sqlite_uint64,
                ),
            >,
            arg3: *mut c_void,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_PROGRESS_HANDLER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_progress_handler(
        arg1: *mut sqlite3,
        arg2: c_int,
        arg3: Option<
            unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
        >,
        arg4: *mut c_void,
    ) {
        let ptr = __SQLITE3_PROGRESS_HANDLER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: c_int,
            arg3: Option<
                unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
            >,
            arg4: *mut c_void,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_REALLOC: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_realloc(
        arg1: *mut c_void,
        arg2: c_int,
    ) -> *mut c_void {
        let ptr = __SQLITE3_REALLOC.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: c_int,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_RESET: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> c_int {
        let ptr = __SQLITE3_RESET.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(pStmt: *mut sqlite3_stmt) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(pStmt)
    }

    static __SQLITE3_RESULT_BLOB: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_blob(
        arg1: *mut sqlite3_context,
        arg2: *const crate::ffi::c_void,
        arg3: c_int,
        arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) {
        let ptr = __SQLITE3_RESULT_BLOB.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: *const crate::ffi::c_void,
            arg3: c_int,
            arg4: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_RESULT_DOUBLE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_double(arg1: *mut sqlite3_context, arg2: f64) {
        let ptr = __SQLITE3_RESULT_DOUBLE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_context, arg2: f64) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1, arg2)
    }

    static __SQLITE3_RESULT_ERROR: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_error(
        arg1: *mut sqlite3_context,
        arg2: *const c_char,
        arg3: c_int,
    ) {
        let ptr = __SQLITE3_RESULT_ERROR.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: *const c_char,
            arg3: c_int,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_RESULT_INT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_int(arg1: *mut sqlite3_context, arg2: crate::ffi::c_int) {
        let ptr = __SQLITE3_RESULT_INT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: c_int,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_RESULT_INT64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_int64(arg1: *mut sqlite3_context, arg2: sqlite_int64) {
        let ptr = __SQLITE3_RESULT_INT64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_context, arg2: sqlite_int64) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1, arg2)
    }

    static __SQLITE3_RESULT_NULL: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_null(arg1: *mut sqlite3_context) {
        let ptr = __SQLITE3_RESULT_NULL.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_context) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_RESULT_TEXT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_text(
        arg1: *mut sqlite3_context,
        arg2: *const c_char,
        arg3: c_int,
        arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) {
        let ptr = __SQLITE3_RESULT_TEXT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: *const c_char,
            arg3: c_int,
            arg4: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_RESULT_VALUE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_value(
        arg1: *mut sqlite3_context,
        arg2: *mut sqlite3_value,
    ) {
        let ptr = __SQLITE3_RESULT_VALUE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: *mut sqlite3_value,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_ROLLBACK_HOOK: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_rollback_hook(
        arg1: *mut sqlite3,
        arg2: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        arg3: *mut c_void,
    ) -> *mut c_void {
        let ptr = __SQLITE3_ROLLBACK_HOOK.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
            arg3: *mut c_void,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_SET_AUTHORIZER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_set_authorizer(
        arg1: *mut sqlite3,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: c_int,
                arg3: *const c_char,
                arg4: *const c_char,
                arg5: *const c_char,
                arg6: *const c_char,
            ) -> c_int,
        >,
        arg3: *mut c_void,
    ) -> c_int {
        let ptr = __SQLITE3_SET_AUTHORIZER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: Option<
                unsafe extern "C" fn(
                    arg1: *mut c_void,
                    arg2: c_int,
                    arg3: *const c_char,
                    arg4: *const c_char,
                    arg5: *const c_char,
                    arg6: *const c_char,
                ) -> c_int,
            >,
            arg3: *mut c_void,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_SET_AUXDATA: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_set_auxdata(
        arg1: *mut sqlite3_context,
        arg2: c_int,
        arg3: *mut c_void,
        arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) {
        let ptr = __SQLITE3_SET_AUXDATA.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: c_int,
            arg3: *mut c_void,
            arg4: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_STEP: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_step(arg1: *mut sqlite3_stmt) -> c_int {
        let ptr = __SQLITE3_STEP.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_TABLE_COLUMN_METADATA: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_table_column_metadata(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: *const c_char,
        arg4: *const c_char,
        arg5: *mut *const c_char,
        arg6: *mut *const c_char,
        arg7: *mut c_int,
        arg8: *mut c_int,
        arg9: *mut c_int,
    ) -> c_int {
        let ptr = __SQLITE3_TABLE_COLUMN_METADATA
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: *const c_char,
            arg4: *const c_char,
            arg5: *mut *const c_char,
            arg6: *mut *const c_char,
            arg7: *mut c_int,
            arg8: *mut c_int,
            arg9: *mut c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
    }

    static __SQLITE3_TOTAL_CHANGES: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_total_changes(arg1: *mut sqlite3) -> c_int {
        let ptr = __SQLITE3_TOTAL_CHANGES.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_TRACE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_trace(
        arg1: *mut sqlite3,
        xTrace: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: *const c_char,
            ),
        >,
        arg2: *mut c_void,
    ) -> *mut c_void {
        let ptr = __SQLITE3_TRACE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            xTrace: Option<
                unsafe extern "C" fn(
                    arg1: *mut c_void,
                    arg2: *const c_char,
                ),
            >,
            arg2: *mut c_void,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, xTrace, arg2)
    }

    static __SQLITE3_UPDATE_HOOK: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_update_hook(
        arg1: *mut sqlite3,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: c_int,
                arg3: *const c_char,
                arg4: *const c_char,
                arg5: sqlite_int64,
            ),
        >,
        arg3: *mut c_void,
    ) -> *mut c_void {
        let ptr = __SQLITE3_UPDATE_HOOK.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: Option<
                unsafe extern "C" fn(
                    arg1: *mut c_void,
                    arg2: c_int,
                    arg3: *const c_char,
                    arg4: *const c_char,
                    arg5: sqlite_int64,
                ),
            >,
            arg3: *mut c_void,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_USER_DATA: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_user_data(arg1: *mut sqlite3_context) -> *mut c_void {
        let ptr = __SQLITE3_USER_DATA.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_VALUE_BLOB: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_blob(
        arg1: *mut sqlite3_value,
    ) -> *const crate::ffi::c_void {
        let ptr = __SQLITE3_VALUE_BLOB.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_value,
        ) -> *const crate::ffi::c_void = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_VALUE_BYTES: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_bytes(arg1: *mut sqlite3_value) -> c_int {
        let ptr = __SQLITE3_VALUE_BYTES.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_VALUE_DOUBLE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_double(arg1: *mut sqlite3_value) -> f64 {
        let ptr = __SQLITE3_VALUE_DOUBLE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_value) -> f64 = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_VALUE_INT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_int(arg1: *mut sqlite3_value) -> c_int {
        let ptr = __SQLITE3_VALUE_INT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_VALUE_INT64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_int64(arg1: *mut sqlite3_value) -> sqlite_int64 {
        let ptr = __SQLITE3_VALUE_INT64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_value) -> sqlite_int64 = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_VALUE_NUMERIC_TYPE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_numeric_type(
        arg1: *mut sqlite3_value,
    ) -> c_int {
        let ptr = __SQLITE3_VALUE_NUMERIC_TYPE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_VALUE_TEXT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_text(
        arg1: *mut sqlite3_value,
    ) -> *const crate::ffi::c_uchar {
        let ptr = __SQLITE3_VALUE_TEXT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_value,
        ) -> *const crate::ffi::c_uchar = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_VALUE_TYPE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_type(arg1: *mut sqlite3_value) -> c_int {
        let ptr = __SQLITE3_VALUE_TYPE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_OVERLOAD_FUNCTION: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_overload_function(
        arg1: *mut sqlite3,
        zFuncName: *const c_char,
        nArg: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_OVERLOAD_FUNCTION.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            zFuncName: *const c_char,
            nArg: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, zFuncName, nArg)
    }

    static __SQLITE3_PREPARE_V2: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_prepare_v2(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: c_int,
        arg4: *mut *mut sqlite3_stmt,
        arg5: *mut *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_PREPARE_V2.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: c_int,
            arg4: *mut *mut sqlite3_stmt,
            arg5: *mut *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5)
    }

    static __SQLITE3_CLEAR_BINDINGS: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_clear_bindings(arg1: *mut sqlite3_stmt) -> c_int {
        let ptr = __SQLITE3_CLEAR_BINDINGS.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_CREATE_MODULE_V2: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_create_module_v2(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: *const sqlite3_module,
        arg4: *mut c_void,
        xDestroy: Option<
            unsafe extern "C" fn(arg1: *mut c_void),
        >,
    ) -> c_int {
        let ptr = __SQLITE3_CREATE_MODULE_V2.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: *const sqlite3_module,
            arg4: *mut c_void,
            xDestroy: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, xDestroy)
    }

    static __SQLITE3_BIND_ZEROBLOB: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_zeroblob(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_ZEROBLOB.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_BLOB_BYTES: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_blob_bytes(arg1: *mut sqlite3_blob) -> c_int {
        let ptr = __SQLITE3_BLOB_BYTES.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_blob) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_BLOB_CLOSE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_blob_close(arg1: *mut sqlite3_blob) -> c_int {
        let ptr = __SQLITE3_BLOB_CLOSE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_blob) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_BLOB_OPEN: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_blob_open(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: *const c_char,
        arg4: *const c_char,
        arg5: sqlite3_int64,
        arg6: c_int,
        arg7: *mut *mut sqlite3_blob,
    ) -> c_int {
        let ptr = __SQLITE3_BLOB_OPEN.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: *const c_char,
            arg4: *const c_char,
            arg5: sqlite3_int64,
            arg6: c_int,
            arg7: *mut *mut sqlite3_blob,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
    }

    static __SQLITE3_BLOB_READ: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_blob_read(
        arg1: *mut sqlite3_blob,
        arg2: *mut c_void,
        arg3: c_int,
        arg4: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_BLOB_READ.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_blob,
            arg2: *mut c_void,
            arg3: c_int,
            arg4: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_BLOB_WRITE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_blob_write(
        arg1: *mut sqlite3_blob,
        arg2: *const crate::ffi::c_void,
        arg3: c_int,
        arg4: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_BLOB_WRITE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_blob,
            arg2: *const crate::ffi::c_void,
            arg3: c_int,
            arg4: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_CREATE_COLLATION_V2: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_create_collation_v2(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: c_int,
        arg4: *mut c_void,
        arg5: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: c_int,
                arg3: *const crate::ffi::c_void,
                arg4: c_int,
                arg5: *const crate::ffi::c_void,
            ) -> c_int,
        >,
        arg6: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) -> c_int {
        let ptr = __SQLITE3_CREATE_COLLATION_V2
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: c_int,
            arg4: *mut c_void,
            arg5: Option<
                unsafe extern "C" fn(
                    arg1: *mut c_void,
                    arg2: c_int,
                    arg3: *const crate::ffi::c_void,
                    arg4: c_int,
                    arg5: *const crate::ffi::c_void,
                ) -> c_int,
            >,
            arg6: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5, arg6)
    }

    static __SQLITE3_FILE_CONTROL: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_file_control(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: c_int,
        arg4: *mut c_void,
    ) -> c_int {
        let ptr = __SQLITE3_FILE_CONTROL.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: c_int,
            arg4: *mut c_void,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_MEMORY_HIGHWATER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_memory_highwater(arg1: crate::ffi::c_int) -> sqlite3_int64 {
        let ptr = __SQLITE3_MEMORY_HIGHWATER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: crate::ffi::c_int) -> sqlite3_int64 = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_MEMORY_USED: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_memory_used() -> sqlite3_int64 {
        let ptr = __SQLITE3_MEMORY_USED.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn() -> sqlite3_int64 = ::core::mem::transmute(ptr);
        (fun)()
    }

    static __SQLITE3_MUTEX_ALLOC: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_mutex_alloc(arg1: crate::ffi::c_int) -> *mut sqlite3_mutex {
        let ptr = __SQLITE3_MUTEX_ALLOC.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: crate::ffi::c_int) -> *mut sqlite3_mutex = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_MUTEX_ENTER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_mutex_enter(arg1: *mut sqlite3_mutex) {
        let ptr = __SQLITE3_MUTEX_ENTER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_mutex) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_MUTEX_FREE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_mutex_free(arg1: *mut sqlite3_mutex) {
        let ptr = __SQLITE3_MUTEX_FREE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_mutex) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_MUTEX_LEAVE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_mutex_leave(arg1: *mut sqlite3_mutex) {
        let ptr = __SQLITE3_MUTEX_LEAVE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_mutex) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_MUTEX_TRY: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_mutex_try(arg1: *mut sqlite3_mutex) -> c_int {
        let ptr = __SQLITE3_MUTEX_TRY.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_mutex) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_OPEN_V2: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_open_v2(
        arg1: *const c_char,
        arg2: *mut *mut sqlite3,
        arg3: c_int,
        arg4: *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_OPEN_V2.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *mut *mut sqlite3,
            arg3: c_int,
            arg4: *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_RELEASE_MEMORY: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_release_memory(arg1: crate::ffi::c_int) -> c_int {
        let ptr = __SQLITE3_RELEASE_MEMORY.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: crate::ffi::c_int) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_RESULT_ERROR_NOMEM: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_error_nomem(arg1: *mut sqlite3_context) {
        let ptr = __SQLITE3_RESULT_ERROR_NOMEM.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_context) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_RESULT_ERROR_TOOBIG: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_error_toobig(arg1: *mut sqlite3_context) {
        let ptr = __SQLITE3_RESULT_ERROR_TOOBIG
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_context) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_SLEEP: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_sleep(arg1: crate::ffi::c_int) -> c_int {
        let ptr = __SQLITE3_SLEEP.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: crate::ffi::c_int) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_SOFT_HEAP_LIMIT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_soft_heap_limit(arg1: crate::ffi::c_int) {
        let ptr = __SQLITE3_SOFT_HEAP_LIMIT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: crate::ffi::c_int) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_VFS_FIND: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_vfs_find(arg1: *const crate::ffi::c_char) -> *mut sqlite3_vfs {
        let ptr = __SQLITE3_VFS_FIND.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
        ) -> *mut sqlite3_vfs = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_VFS_REGISTER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_vfs_register(
        arg1: *mut sqlite3_vfs,
        arg2: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_VFS_REGISTER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_vfs,
            arg2: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_VFS_UNREGISTER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_vfs_unregister(arg1: *mut sqlite3_vfs) -> c_int {
        let ptr = __SQLITE3_VFS_UNREGISTER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_vfs) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_THREADSAFE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_threadsafe() -> c_int {
        let ptr = __SQLITE3_THREADSAFE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn() -> c_int = ::core::mem::transmute(ptr);
        (fun)()
    }

    static __SQLITE3_RESULT_ZEROBLOB: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_zeroblob(
        arg1: *mut sqlite3_context,
        arg2: c_int,
    ) {
        let ptr = __SQLITE3_RESULT_ZEROBLOB.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: c_int,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_RESULT_ERROR_CODE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_error_code(
        arg1: *mut sqlite3_context,
        arg2: c_int,
    ) {
        let ptr = __SQLITE3_RESULT_ERROR_CODE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: c_int,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_RANDOMNESS: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_randomness(
        arg1: c_int,
        arg2: *mut c_void,
    ) {
        let ptr = __SQLITE3_RANDOMNESS.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: c_int,
            arg2: *mut c_void,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_CONTEXT_DB_HANDLE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_context_db_handle(arg1: *mut sqlite3_context) -> *mut sqlite3 {
        let ptr = __SQLITE3_CONTEXT_DB_HANDLE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_context) -> *mut sqlite3 = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_EXTENDED_RESULT_CODES: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_extended_result_codes(
        arg1: *mut sqlite3,
        arg2: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_EXTENDED_RESULT_CODES
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_LIMIT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_limit(
        arg1: *mut sqlite3,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_LIMIT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: c_int,
            arg3: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_NEXT_STMT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_next_stmt(
        arg1: *mut sqlite3,
        arg2: *mut sqlite3_stmt,
    ) -> *mut sqlite3_stmt {
        let ptr = __SQLITE3_NEXT_STMT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *mut sqlite3_stmt,
        ) -> *mut sqlite3_stmt = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_SQL: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_sql(arg1: *mut sqlite3_stmt) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_SQL.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_STATUS: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_status(
        arg1: c_int,
        arg2: *mut c_int,
        arg3: *mut c_int,
        arg4: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_STATUS.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: c_int,
            arg2: *mut c_int,
            arg3: *mut c_int,
            arg4: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_BACKUP_FINISH: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_backup_finish(arg1: *mut sqlite3_backup) -> c_int {
        let ptr = __SQLITE3_BACKUP_FINISH.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_backup) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_BACKUP_INIT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_backup_init(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: *mut sqlite3,
        arg4: *const c_char,
    ) -> *mut sqlite3_backup {
        let ptr = __SQLITE3_BACKUP_INIT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: *mut sqlite3,
            arg4: *const c_char,
        ) -> *mut sqlite3_backup = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_BACKUP_PAGECOUNT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_backup_pagecount(arg1: *mut sqlite3_backup) -> c_int {
        let ptr = __SQLITE3_BACKUP_PAGECOUNT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_backup) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_BACKUP_REMAINING: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_backup_remaining(arg1: *mut sqlite3_backup) -> c_int {
        let ptr = __SQLITE3_BACKUP_REMAINING.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_backup) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_BACKUP_STEP: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_backup_step(
        arg1: *mut sqlite3_backup,
        arg2: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_BACKUP_STEP.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_backup,
            arg2: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_COMPILEOPTION_GET: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_compileoption_get(
        arg1: c_int,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_COMPILEOPTION_GET.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: c_int,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_COMPILEOPTION_USED: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_compileoption_used(
        arg1: *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_COMPILEOPTION_USED.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_CREATE_FUNCTION_V2: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_create_function_v2(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: c_int,
        arg4: c_int,
        arg5: *mut c_void,
        xFunc: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: c_int,
                arg3: *mut *mut sqlite3_value,
            ),
        >,
        xStep: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: c_int,
                arg3: *mut *mut sqlite3_value,
            ),
        >,
        xFinal: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
        xDestroy: Option<
            unsafe extern "C" fn(arg1: *mut c_void),
        >,
    ) -> c_int {
        let ptr = __SQLITE3_CREATE_FUNCTION_V2.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: c_int,
            arg4: c_int,
            arg5: *mut c_void,
            xFunc: Option<
                unsafe extern "C" fn(
                    arg1: *mut sqlite3_context,
                    arg2: c_int,
                    arg3: *mut *mut sqlite3_value,
                ),
            >,
            xStep: Option<
                unsafe extern "C" fn(
                    arg1: *mut sqlite3_context,
                    arg2: c_int,
                    arg3: *mut *mut sqlite3_value,
                ),
            >,
            xFinal: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
            xDestroy: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5, xFunc, xStep, xFinal, xDestroy)
    }

    static __SQLITE3_DB_CONFIG: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_db_config(
        arg1: *mut sqlite3,
        arg2: c_int,
        arg3: c_int,
        arg4: *mut c_int,
    ) -> c_int {
        let ptr = __SQLITE3_DB_CONFIG.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: c_int,
            ...
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_DB_MUTEX: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_db_mutex(arg1: *mut sqlite3) -> *mut sqlite3_mutex {
        let ptr = __SQLITE3_DB_MUTEX.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> *mut sqlite3_mutex = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_DB_STATUS: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_db_status(
        arg1: *mut sqlite3,
        arg2: c_int,
        arg3: *mut c_int,
        arg4: *mut c_int,
        arg5: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_DB_STATUS.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: c_int,
            arg3: *mut c_int,
            arg4: *mut c_int,
            arg5: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5)
    }

    static __SQLITE3_EXTENDED_ERRCODE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_extended_errcode(arg1: *mut sqlite3) -> c_int {
        let ptr = __SQLITE3_EXTENDED_ERRCODE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_LOG: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_log(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *const c_char,
    ) {
        let ptr = __SQLITE3_LOG.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized");
        let fun: unsafe extern "C" fn(
            arg1: c_int,
            arg2: *const c_char,
            ...
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_SOFT_HEAP_LIMIT64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_soft_heap_limit64(arg1: sqlite3_int64) -> sqlite3_int64 {
        let ptr = __SQLITE3_SOFT_HEAP_LIMIT64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: sqlite3_int64) -> sqlite3_int64 = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_SOURCEID: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_sourceid() -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_SOURCEID.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn() -> *const crate::ffi::c_char = ::core::mem::transmute(
            ptr,
        );
        (fun)()
    }

    static __SQLITE3_STMT_STATUS: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_stmt_status(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_STMT_STATUS.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_STRNICMP: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_strnicmp(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_STRNICMP.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *const c_char,
            arg3: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_UNLOCK_NOTIFY: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_unlock_notify(
        arg1: *mut sqlite3,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: *mut *mut c_void,
                arg2: c_int,
            ),
        >,
        arg3: *mut c_void,
    ) -> c_int {
        let ptr = __SQLITE3_UNLOCK_NOTIFY.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: Option<
                unsafe extern "C" fn(
                    arg1: *mut *mut c_void,
                    arg2: c_int,
                ),
            >,
            arg3: *mut c_void,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_WAL_AUTOCHECKPOINT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_wal_autocheckpoint(
        arg1: *mut sqlite3,
        arg2: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_WAL_AUTOCHECKPOINT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_WAL_CHECKPOINT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_wal_checkpoint(
        arg1: *mut sqlite3,
        arg2: *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_WAL_CHECKPOINT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_WAL_HOOK: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_wal_hook(
        arg1: *mut sqlite3,
        arg2: Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: *mut sqlite3,
                arg3: *const c_char,
                arg4: c_int,
            ) -> c_int,
        >,
        arg3: *mut c_void,
    ) -> *mut c_void {
        let ptr = __SQLITE3_WAL_HOOK.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: Option<
                unsafe extern "C" fn(
                    arg1: *mut c_void,
                    arg2: *mut sqlite3,
                    arg3: *const c_char,
                    arg4: c_int,
                ) -> c_int,
            >,
            arg3: *mut c_void,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_BLOB_REOPEN: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_blob_reopen(
        arg1: *mut sqlite3_blob,
        arg2: sqlite3_int64,
    ) -> c_int {
        let ptr = __SQLITE3_BLOB_REOPEN.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_blob,
            arg2: sqlite3_int64,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_VTAB_CONFIG: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_vtab_config(
        arg1: *mut sqlite3,
        op: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_VTAB_CONFIG.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            op: c_int,
            ...
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, op)
    }

    static __SQLITE3_VTAB_ON_CONFLICT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_vtab_on_conflict(arg1: *mut sqlite3) -> c_int {
        let ptr = __SQLITE3_VTAB_ON_CONFLICT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_DB_FILENAME: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_db_filename(
        arg1: *mut sqlite3,
        arg2: *const c_char,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_DB_FILENAME.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_DB_READONLY: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_db_readonly(
        arg1: *mut sqlite3,
        arg2: *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_DB_READONLY.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_DB_RELEASE_MEMORY: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_db_release_memory(arg1: *mut sqlite3) -> c_int {
        let ptr = __SQLITE3_DB_RELEASE_MEMORY.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_ERRSTR: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_errstr(arg1: crate::ffi::c_int) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_ERRSTR.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: c_int,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_STMT_BUSY: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_stmt_busy(arg1: *mut sqlite3_stmt) -> c_int {
        let ptr = __SQLITE3_STMT_BUSY.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_STMT_READONLY: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_stmt_readonly(arg1: *mut sqlite3_stmt) -> c_int {
        let ptr = __SQLITE3_STMT_READONLY.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_STRICMP: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_stricmp(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_STRICMP.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_URI_BOOLEAN: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_uri_boolean(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_URI_BOOLEAN.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *const c_char,
            arg3: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_URI_INT64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_uri_int64(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: sqlite3_int64,
    ) -> sqlite3_int64 {
        let ptr = __SQLITE3_URI_INT64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *const c_char,
            arg3: sqlite3_int64,
        ) -> sqlite3_int64 = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_URI_PARAMETER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_uri_parameter(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_URI_PARAMETER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *const c_char,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_WAL_CHECKPOINT_V2: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_wal_checkpoint_v2(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: c_int,
        arg4: *mut c_int,
        arg5: *mut c_int,
    ) -> c_int {
        let ptr = __SQLITE3_WAL_CHECKPOINT_V2.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: c_int,
            arg4: *mut c_int,
            arg5: *mut c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5)
    }

    static __SQLITE3_AUTO_EXTENSION: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_auto_extension(
        arg1: Option<unsafe extern "C" fn()>,
    ) -> c_int {
        let ptr = __SQLITE3_AUTO_EXTENSION.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: Option<unsafe extern "C" fn()>,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_BIND_BLOB64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_blob64(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: *const crate::ffi::c_void,
        arg4: sqlite3_uint64,
        arg5: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_BLOB64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: *const crate::ffi::c_void,
            arg4: sqlite3_uint64,
            arg5: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5)
    }

    static __SQLITE3_BIND_TEXT64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_text64(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: *const c_char,
        arg4: sqlite3_uint64,
        arg5: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        arg6: crate::ffi::c_uchar,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_TEXT64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: *const c_char,
            arg4: sqlite3_uint64,
            arg5: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
            arg6: crate::ffi::c_uchar,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5, arg6)
    }

    static __SQLITE3_CANCEL_AUTO_EXTENSION: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_cancel_auto_extension(
        arg1: Option<unsafe extern "C" fn()>,
    ) -> c_int {
        let ptr = __SQLITE3_CANCEL_AUTO_EXTENSION
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: Option<unsafe extern "C" fn()>,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_LOAD_EXTENSION: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_load_extension(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: *const c_char,
        arg4: *mut *mut c_char,
    ) -> c_int {
        let ptr = __SQLITE3_LOAD_EXTENSION.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: *const c_char,
            arg4: *mut *mut c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_MALLOC64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_malloc64(arg1: sqlite3_uint64) -> *mut c_void {
        let ptr = __SQLITE3_MALLOC64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: sqlite3_uint64) -> *mut c_void = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_MSIZE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_msize(arg1: *mut c_void) -> sqlite3_uint64 {
        let ptr = __SQLITE3_MSIZE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut c_void) -> sqlite3_uint64 = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_REALLOC64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_realloc64(
        arg1: *mut c_void,
        arg2: sqlite3_uint64,
    ) -> *mut c_void {
        let ptr = __SQLITE3_REALLOC64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: sqlite3_uint64,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_RESET_AUTO_EXTENSION: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_reset_auto_extension() {
        let ptr = __SQLITE3_RESET_AUTO_EXTENSION
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn() = ::core::mem::transmute(ptr);
        (fun)()
    }

    static __SQLITE3_RESULT_BLOB64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_blob64(
        arg1: *mut sqlite3_context,
        arg2: *const crate::ffi::c_void,
        arg3: sqlite3_uint64,
        arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) {
        let ptr = __SQLITE3_RESULT_BLOB64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: *const crate::ffi::c_void,
            arg3: sqlite3_uint64,
            arg4: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_RESULT_TEXT64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_text64(
        arg1: *mut sqlite3_context,
        arg2: *const c_char,
        arg3: sqlite3_uint64,
        arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        arg5: crate::ffi::c_uchar,
    ) {
        let ptr = __SQLITE3_RESULT_TEXT64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: *const c_char,
            arg3: sqlite3_uint64,
            arg4: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
            arg5: crate::ffi::c_uchar,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5)
    }

    static __SQLITE3_STRGLOB: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_strglob(
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_STRGLOB.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_VALUE_DUP: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_dup(arg1: *const sqlite3_value) -> *mut sqlite3_value {
        let ptr = __SQLITE3_VALUE_DUP.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *const sqlite3_value) -> *mut sqlite3_value = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_VALUE_FREE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_free(arg1: *mut sqlite3_value) {
        let ptr = __SQLITE3_VALUE_FREE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_value) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_RESULT_ZEROBLOB64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_zeroblob64(
        arg1: *mut sqlite3_context,
        arg2: sqlite3_uint64,
    ) -> c_int {
        let ptr = __SQLITE3_RESULT_ZEROBLOB64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: sqlite3_uint64,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_BIND_ZEROBLOB64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_zeroblob64(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: sqlite3_uint64,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_ZEROBLOB64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: sqlite3_uint64,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_VALUE_SUBTYPE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_subtype(arg1: *mut sqlite3_value) -> ::core::ffi::c_uint {
        let ptr = __SQLITE3_VALUE_SUBTYPE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_value) -> ::core::ffi::c_uint = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_RESULT_SUBTYPE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_subtype(
        arg1: *mut sqlite3_context,
        arg2: crate::ffi::c_uint,
    ) {
        let ptr = __SQLITE3_RESULT_SUBTYPE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: crate::ffi::c_uint,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_STATUS64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_status64(
        arg1: c_int,
        arg2: *mut sqlite3_int64,
        arg3: *mut sqlite3_int64,
        arg4: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_STATUS64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: c_int,
            arg2: *mut sqlite3_int64,
            arg3: *mut sqlite3_int64,
            arg4: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_STRLIKE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_strlike(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: crate::ffi::c_uint,
    ) -> c_int {
        let ptr = __SQLITE3_STRLIKE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *const c_char,
            arg3: crate::ffi::c_uint,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_DB_CACHEFLUSH: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_db_cacheflush(arg1: *mut sqlite3) -> c_int {
        let ptr = __SQLITE3_DB_CACHEFLUSH.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_SYSTEM_ERRNO: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_system_errno(arg1: *mut sqlite3) -> c_int {
        let ptr = __SQLITE3_SYSTEM_ERRNO.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_TRACE_V2: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_trace_v2(
        arg1: *mut sqlite3,
        arg2: crate::ffi::c_uint,
        arg3: Option<
            unsafe extern "C" fn(
                arg1: crate::ffi::c_uint,
                arg2: *mut c_void,
                arg3: *mut c_void,
                arg4: *mut c_void,
            ) -> c_int,
        >,
        arg4: *mut c_void,
    ) -> c_int {
        let ptr = __SQLITE3_TRACE_V2.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: crate::ffi::c_uint,
            arg3: Option<
                unsafe extern "C" fn(
                    arg1: crate::ffi::c_uint,
                    arg2: *mut c_void,
                    arg3: *mut c_void,
                    arg4: *mut c_void,
                ) -> c_int,
            >,
            arg4: *mut c_void,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_EXPANDED_SQL: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_expanded_sql(arg1: *mut sqlite3_stmt) -> *mut crate::ffi::c_char {
        let ptr = __SQLITE3_EXPANDED_SQL.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> *mut crate::ffi::c_char = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_SET_LAST_INSERT_ROWID: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_set_last_insert_rowid(arg1: *mut sqlite3, arg2: sqlite3_int64) {
        let ptr = __SQLITE3_SET_LAST_INSERT_ROWID
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3, arg2: sqlite3_int64) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1, arg2)
    }

    static __SQLITE3_PREPARE_V3: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_prepare_v3(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: c_int,
        arg4: crate::ffi::c_uint,
        arg5: *mut *mut sqlite3_stmt,
        arg6: *mut *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_PREPARE_V3.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: c_int,
            arg4: crate::ffi::c_uint,
            arg5: *mut *mut sqlite3_stmt,
            arg6: *mut *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5, arg6)
    }

    static __SQLITE3_BIND_POINTER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_bind_pointer(
        arg1: *mut sqlite3_stmt,
        arg2: c_int,
        arg3: *mut c_void,
        arg4: *const c_char,
        arg5: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) -> c_int {
        let ptr = __SQLITE3_BIND_POINTER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
            arg2: c_int,
            arg3: *mut c_void,
            arg4: *const c_char,
            arg5: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5)
    }

    static __SQLITE3_RESULT_POINTER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_result_pointer(
        arg1: *mut sqlite3_context,
        arg2: *mut c_void,
        arg3: *const c_char,
        arg4: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    ) {
        let ptr = __SQLITE3_RESULT_POINTER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: *mut c_void,
            arg3: *const c_char,
            arg4: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4)
    }

    static __SQLITE3_VALUE_POINTER: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_pointer(
        arg1: *mut sqlite3_value,
        arg2: *const c_char,
    ) -> *mut c_void {
        let ptr = __SQLITE3_VALUE_POINTER.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_value,
            arg2: *const c_char,
        ) -> *mut c_void = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_VTAB_NOCHANGE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_vtab_nochange(arg1: *mut sqlite3_context) -> c_int {
        let ptr = __SQLITE3_VTAB_NOCHANGE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_context) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_VALUE_NOCHANGE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_nochange(arg1: *mut sqlite3_value) -> c_int {
        let ptr = __SQLITE3_VALUE_NOCHANGE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_VTAB_COLLATION: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_vtab_collation(
        arg1: *mut sqlite3_index_info,
        arg2: c_int,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_VTAB_COLLATION.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_index_info,
            arg2: c_int,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_KEYWORD_COUNT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_keyword_count() -> c_int {
        let ptr = __SQLITE3_KEYWORD_COUNT.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn() -> c_int = ::core::mem::transmute(ptr);
        (fun)()
    }

    static __SQLITE3_KEYWORD_NAME: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_keyword_name(
        arg1: c_int,
        arg2: *mut *const c_char,
        arg3: *mut c_int,
    ) -> c_int {
        let ptr = __SQLITE3_KEYWORD_NAME.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: c_int,
            arg2: *mut *const c_char,
            arg3: *mut c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3)
    }

    static __SQLITE3_KEYWORD_CHECK: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_keyword_check(
        arg1: *const c_char,
        arg2: c_int,
    ) -> c_int {
        let ptr = __SQLITE3_KEYWORD_CHECK.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: c_int,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_STR_NEW: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_str_new(arg1: *mut sqlite3) -> *mut sqlite3_str {
        let ptr = __SQLITE3_STR_NEW.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3) -> *mut sqlite3_str = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_STR_FINISH: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_str_finish(arg1: *mut sqlite3_str) -> *mut crate::ffi::c_char {
        let ptr = __SQLITE3_STR_FINISH.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_str) -> *mut crate::ffi::c_char = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_STR_APPEND: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_str_append(
        arg1: *mut sqlite3_str,
        zIn: *const c_char,
        N: c_int,
    ) {
        let ptr = __SQLITE3_STR_APPEND.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_str,
            zIn: *const c_char,
            N: c_int,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, zIn, N)
    }

    static __SQLITE3_STR_APPENDALL: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_str_appendall(
        arg1: *mut sqlite3_str,
        zIn: *const c_char,
    ) {
        let ptr = __SQLITE3_STR_APPENDALL.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_str,
            zIn: *const c_char,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, zIn)
    }

    static __SQLITE3_STR_APPENDCHAR: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_str_appendchar(
        arg1: *mut sqlite3_str,
        N: c_int,
        C: c_char,
    ) {
        let ptr = __SQLITE3_STR_APPENDCHAR.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_str,
            N: c_int,
            C: c_char,
        ) = ::core::mem::transmute(ptr);
        (fun)(arg1, N, C)
    }

    static __SQLITE3_STR_RESET: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_str_reset(arg1: *mut sqlite3_str) {
        let ptr = __SQLITE3_STR_RESET.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_str) = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_STR_ERRCODE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_str_errcode(arg1: *mut sqlite3_str) -> c_int {
        let ptr = __SQLITE3_STR_ERRCODE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_str) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_STR_LENGTH: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_str_length(arg1: *mut sqlite3_str) -> c_int {
        let ptr = __SQLITE3_STR_LENGTH.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_str) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_STR_VALUE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_str_value(arg1: *mut sqlite3_str) -> *mut crate::ffi::c_char {
        let ptr = __SQLITE3_STR_VALUE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_str) -> *mut crate::ffi::c_char = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_CREATE_WINDOW_FUNCTION: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_create_window_function(
        arg1: *mut sqlite3,
        arg2: *const c_char,
        arg3: c_int,
        arg4: c_int,
        arg5: *mut c_void,
        xStep: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: c_int,
                arg3: *mut *mut sqlite3_value,
            ),
        >,
        xFinal: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
        xValue: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
        xInv: Option<
            unsafe extern "C" fn(
                arg1: *mut sqlite3_context,
                arg2: c_int,
                arg3: *mut *mut sqlite3_value,
            ),
        >,
        xDestroy: Option<
            unsafe extern "C" fn(arg1: *mut c_void),
        >,
    ) -> c_int {
        let ptr = __SQLITE3_CREATE_WINDOW_FUNCTION
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
            arg3: c_int,
            arg4: c_int,
            arg5: *mut c_void,
            xStep: Option<
                unsafe extern "C" fn(
                    arg1: *mut sqlite3_context,
                    arg2: c_int,
                    arg3: *mut *mut sqlite3_value,
                ),
            >,
            xFinal: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
            xValue: Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
            xInv: Option<
                unsafe extern "C" fn(
                    arg1: *mut sqlite3_context,
                    arg2: c_int,
                    arg3: *mut *mut sqlite3_value,
                ),
            >,
            xDestroy: Option<
                unsafe extern "C" fn(arg1: *mut c_void),
            >,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5, xStep, xFinal, xValue, xInv, xDestroy)
    }

    static __SQLITE3_NORMALIZED_SQL: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_normalized_sql(
        arg1: *mut sqlite3_stmt,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_NORMALIZED_SQL.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3_stmt,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_STMT_ISEXPLAIN: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_stmt_isexplain(arg1: *mut sqlite3_stmt) -> c_int {
        let ptr = __SQLITE3_STMT_ISEXPLAIN.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_stmt) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_VALUE_FROMBIND: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_value_frombind(arg1: *mut sqlite3_value) -> c_int {
        let ptr = __SQLITE3_VALUE_FROMBIND.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut sqlite3_value) -> c_int = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_DROP_MODULES: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_drop_modules(
        arg1: *mut sqlite3,
        arg2: *mut *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_DROP_MODULES.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *mut *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_HARD_HEAP_LIMIT64: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_hard_heap_limit64(arg1: sqlite3_int64) -> sqlite3_int64 {
        let ptr = __SQLITE3_HARD_HEAP_LIMIT64.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: sqlite3_int64) -> sqlite3_int64 = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_URI_KEY: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_uri_key(
        arg1: *const c_char,
        arg2: c_int,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_URI_KEY.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: c_int,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    static __SQLITE3_FILENAME_DATABASE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_filename_database(
        arg1: *const c_char,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_FILENAME_DATABASE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_FILENAME_JOURNAL: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_filename_journal(
        arg1: *const c_char,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_FILENAME_JOURNAL.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_FILENAME_WAL: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_filename_wal(
        arg1: *const c_char,
    ) -> *const crate::ffi::c_char {
        let ptr = __SQLITE3_FILENAME_WAL.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
        ) -> *const crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_CREATE_FILENAME: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_create_filename(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: *const c_char,
        arg4: c_int,
        arg5: *mut *const c_char,
    ) -> *mut crate::ffi::c_char {
        let ptr = __SQLITE3_CREATE_FILENAME.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
            arg2: *const c_char,
            arg3: *const c_char,
            arg4: c_int,
            arg5: *mut *const c_char,
        ) -> *mut crate::ffi::c_char = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2, arg3, arg4, arg5)
    }

    static __SQLITE3_FREE_FILENAME: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_free_filename(arg1: *mut crate::ffi::c_char) {
        let ptr = __SQLITE3_FREE_FILENAME.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(arg1: *mut crate::ffi::c_char) = ::core::mem::transmute(
            ptr,
        );
        (fun)(arg1)
    }

    static __SQLITE3_DATABASE_FILE_OBJECT: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_database_file_object(
        arg1: *const c_char,
    ) -> *mut sqlite3_file {
        let ptr = __SQLITE3_DATABASE_FILE_OBJECT
            .load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *const c_char,
        ) -> *mut sqlite3_file = ::core::mem::transmute(ptr);
        (fun)(arg1)
    }

    static __SQLITE3_TXN_STATE: ::core::sync::atomic::AtomicPtr<()> = ::core::sync::atomic::AtomicPtr::new(
        ::core::ptr::null_mut(),
    );
    pub unsafe fn sqlite3_txn_state(
        arg1: *mut sqlite3,
        arg2: *const c_char,
    ) -> c_int {
        let ptr = __SQLITE3_TXN_STATE.load(::core::sync::atomic::Ordering::Acquire);
        assert!(! ptr.is_null(), "SQLite API not initialized or SQLite feature omitted");
        let fun: unsafe extern "C" fn(
            arg1: *mut sqlite3,
            arg2: *const c_char,
        ) -> c_int = ::core::mem::transmute(ptr);
        (fun)(arg1, arg2)
    }

    /// Like SQLITE_EXTENSION_INIT2 macro
    pub unsafe fn rusqlite_extension_init2(
        p_api: *mut sqlite3_api_routines,
    ) -> ::core::result::Result<(), crate::InitError> {
        if let Some(fun) = (*p_api).malloc {
            __SQLITE3_MALLOC
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).libversion_number {
            let version = fun();
            if SQLITE_VERSION_NUMBER > version {
                return Err(crate::InitError::VersionMismatch {
                    compile_time: SQLITE_VERSION_NUMBER,
                    runtime: version,
                });
            }
        } else {
            return Err(crate::InitError::NullFunctionPointer);
        }
        if let Some(fun) = (*p_api).aggregate_context {
            __SQLITE3_AGGREGATE_CONTEXT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_blob {
            __SQLITE3_BIND_BLOB
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_double {
            __SQLITE3_BIND_DOUBLE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_int {
            __SQLITE3_BIND_INT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_int64 {
            __SQLITE3_BIND_INT64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_null {
            __SQLITE3_BIND_NULL
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_parameter_count {
            __SQLITE3_BIND_PARAMETER_COUNT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_parameter_index {
            __SQLITE3_BIND_PARAMETER_INDEX
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_parameter_name {
            __SQLITE3_BIND_PARAMETER_NAME
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_text {
            __SQLITE3_BIND_TEXT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_value {
            __SQLITE3_BIND_VALUE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).busy_handler {
            __SQLITE3_BUSY_HANDLER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).busy_timeout {
            __SQLITE3_BUSY_TIMEOUT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).changes {
            __SQLITE3_CHANGES
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).close {
            __SQLITE3_CLOSE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).collation_needed {
            __SQLITE3_COLLATION_NEEDED
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_blob {
            __SQLITE3_COLUMN_BLOB
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_bytes {
            __SQLITE3_COLUMN_BYTES
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_count {
            __SQLITE3_COLUMN_COUNT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_database_name {
            __SQLITE3_COLUMN_DATABASE_NAME
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_decltype {
            __SQLITE3_COLUMN_DECLTYPE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_double {
            __SQLITE3_COLUMN_DOUBLE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_int {
            __SQLITE3_COLUMN_INT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_int64 {
            __SQLITE3_COLUMN_INT64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_name {
            __SQLITE3_COLUMN_NAME
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_origin_name {
            __SQLITE3_COLUMN_ORIGIN_NAME
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_table_name {
            __SQLITE3_COLUMN_TABLE_NAME
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_text {
            __SQLITE3_COLUMN_TEXT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_type {
            __SQLITE3_COLUMN_TYPE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).column_value {
            __SQLITE3_COLUMN_VALUE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).commit_hook {
            __SQLITE3_COMMIT_HOOK
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).complete {
            __SQLITE3_COMPLETE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).data_count {
            __SQLITE3_DATA_COUNT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).db_handle {
            __SQLITE3_DB_HANDLE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).declare_vtab {
            __SQLITE3_DECLARE_VTAB
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).enable_shared_cache {
            __SQLITE3_ENABLE_SHARED_CACHE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).errcode {
            __SQLITE3_ERRCODE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).errmsg {
            __SQLITE3_ERRMSG
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).exec {
            __SQLITE3_EXEC
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).finalize {
            __SQLITE3_FINALIZE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).free {
            __SQLITE3_FREE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).free_table {
            __SQLITE3_FREE_TABLE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).get_autocommit {
            __SQLITE3_GET_AUTOCOMMIT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).get_auxdata {
            __SQLITE3_GET_AUXDATA
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).get_table {
            __SQLITE3_GET_TABLE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).interruptx {
            __SQLITE3_INTERRUPT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).last_insert_rowid {
            __SQLITE3_LAST_INSERT_ROWID
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).libversion {
            __SQLITE3_LIBVERSION
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).libversion_number {
            __SQLITE3_LIBVERSION_NUMBER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).open {
            __SQLITE3_OPEN
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).profile {
            __SQLITE3_PROFILE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).progress_handler {
            __SQLITE3_PROGRESS_HANDLER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).realloc {
            __SQLITE3_REALLOC
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).reset {
            __SQLITE3_RESET
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_blob {
            __SQLITE3_RESULT_BLOB
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_double {
            __SQLITE3_RESULT_DOUBLE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_error {
            __SQLITE3_RESULT_ERROR
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_int {
            __SQLITE3_RESULT_INT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_int64 {
            __SQLITE3_RESULT_INT64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_null {
            __SQLITE3_RESULT_NULL
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_text {
            __SQLITE3_RESULT_TEXT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_value {
            __SQLITE3_RESULT_VALUE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).rollback_hook {
            __SQLITE3_ROLLBACK_HOOK
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).set_authorizer {
            __SQLITE3_SET_AUTHORIZER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).set_auxdata {
            __SQLITE3_SET_AUXDATA
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).step {
            __SQLITE3_STEP
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).table_column_metadata {
            __SQLITE3_TABLE_COLUMN_METADATA
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).total_changes {
            __SQLITE3_TOTAL_CHANGES
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).trace {
            __SQLITE3_TRACE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).update_hook {
            __SQLITE3_UPDATE_HOOK
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).user_data {
            __SQLITE3_USER_DATA
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_blob {
            __SQLITE3_VALUE_BLOB
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_bytes {
            __SQLITE3_VALUE_BYTES
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_double {
            __SQLITE3_VALUE_DOUBLE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_int {
            __SQLITE3_VALUE_INT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_int64 {
            __SQLITE3_VALUE_INT64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_numeric_type {
            __SQLITE3_VALUE_NUMERIC_TYPE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_text {
            __SQLITE3_VALUE_TEXT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_type {
            __SQLITE3_VALUE_TYPE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).overload_function {
            __SQLITE3_OVERLOAD_FUNCTION
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).prepare_v2 {
            __SQLITE3_PREPARE_V2
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).clear_bindings {
            __SQLITE3_CLEAR_BINDINGS
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).create_module_v2 {
            __SQLITE3_CREATE_MODULE_V2
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_zeroblob {
            __SQLITE3_BIND_ZEROBLOB
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).blob_bytes {
            __SQLITE3_BLOB_BYTES
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).blob_close {
            __SQLITE3_BLOB_CLOSE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).blob_open {
            __SQLITE3_BLOB_OPEN
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).blob_read {
            __SQLITE3_BLOB_READ
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).blob_write {
            __SQLITE3_BLOB_WRITE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).create_collation_v2 {
            __SQLITE3_CREATE_COLLATION_V2
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).file_control {
            __SQLITE3_FILE_CONTROL
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).memory_highwater {
            __SQLITE3_MEMORY_HIGHWATER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).memory_used {
            __SQLITE3_MEMORY_USED
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).mutex_alloc {
            __SQLITE3_MUTEX_ALLOC
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).mutex_enter {
            __SQLITE3_MUTEX_ENTER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).mutex_free {
            __SQLITE3_MUTEX_FREE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).mutex_leave {
            __SQLITE3_MUTEX_LEAVE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).mutex_try {
            __SQLITE3_MUTEX_TRY
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).open_v2 {
            __SQLITE3_OPEN_V2
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).release_memory {
            __SQLITE3_RELEASE_MEMORY
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_error_nomem {
            __SQLITE3_RESULT_ERROR_NOMEM
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_error_toobig {
            __SQLITE3_RESULT_ERROR_TOOBIG
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).sleep {
            __SQLITE3_SLEEP
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).soft_heap_limit {
            __SQLITE3_SOFT_HEAP_LIMIT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).vfs_find {
            __SQLITE3_VFS_FIND
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).vfs_register {
            __SQLITE3_VFS_REGISTER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).vfs_unregister {
            __SQLITE3_VFS_UNREGISTER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).xthreadsafe {
            __SQLITE3_THREADSAFE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_zeroblob {
            __SQLITE3_RESULT_ZEROBLOB
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_error_code {
            __SQLITE3_RESULT_ERROR_CODE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).randomness {
            __SQLITE3_RANDOMNESS
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).context_db_handle {
            __SQLITE3_CONTEXT_DB_HANDLE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).extended_result_codes {
            __SQLITE3_EXTENDED_RESULT_CODES
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).limit {
            __SQLITE3_LIMIT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).next_stmt {
            __SQLITE3_NEXT_STMT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).sql {
            __SQLITE3_SQL
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).status {
            __SQLITE3_STATUS
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).backup_finish {
            __SQLITE3_BACKUP_FINISH
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).backup_init {
            __SQLITE3_BACKUP_INIT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).backup_pagecount {
            __SQLITE3_BACKUP_PAGECOUNT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).backup_remaining {
            __SQLITE3_BACKUP_REMAINING
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).backup_step {
            __SQLITE3_BACKUP_STEP
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).compileoption_get {
            __SQLITE3_COMPILEOPTION_GET
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).compileoption_used {
            __SQLITE3_COMPILEOPTION_USED
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).create_function_v2 {
            __SQLITE3_CREATE_FUNCTION_V2
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).db_config {
            __SQLITE3_DB_CONFIG
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).db_mutex {
            __SQLITE3_DB_MUTEX
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).db_status {
            __SQLITE3_DB_STATUS
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).extended_errcode {
            __SQLITE3_EXTENDED_ERRCODE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).log {
            __SQLITE3_LOG
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).soft_heap_limit64 {
            __SQLITE3_SOFT_HEAP_LIMIT64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).sourceid {
            __SQLITE3_SOURCEID
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).stmt_status {
            __SQLITE3_STMT_STATUS
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).strnicmp {
            __SQLITE3_STRNICMP
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).unlock_notify {
            __SQLITE3_UNLOCK_NOTIFY
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).wal_autocheckpoint {
            __SQLITE3_WAL_AUTOCHECKPOINT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).wal_checkpoint {
            __SQLITE3_WAL_CHECKPOINT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).wal_hook {
            __SQLITE3_WAL_HOOK
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).blob_reopen {
            __SQLITE3_BLOB_REOPEN
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).vtab_config {
            __SQLITE3_VTAB_CONFIG
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).vtab_on_conflict {
            __SQLITE3_VTAB_ON_CONFLICT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).db_filename {
            __SQLITE3_DB_FILENAME
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).db_readonly {
            __SQLITE3_DB_READONLY
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).db_release_memory {
            __SQLITE3_DB_RELEASE_MEMORY
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).errstr {
            __SQLITE3_ERRSTR
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).stmt_busy {
            __SQLITE3_STMT_BUSY
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).stmt_readonly {
            __SQLITE3_STMT_READONLY
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).stricmp {
            __SQLITE3_STRICMP
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).uri_boolean {
            __SQLITE3_URI_BOOLEAN
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).uri_int64 {
            __SQLITE3_URI_INT64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).uri_parameter {
            __SQLITE3_URI_PARAMETER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).wal_checkpoint_v2 {
            __SQLITE3_WAL_CHECKPOINT_V2
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).auto_extension {
            __SQLITE3_AUTO_EXTENSION
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_blob64 {
            __SQLITE3_BIND_BLOB64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_text64 {
            __SQLITE3_BIND_TEXT64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).cancel_auto_extension {
            __SQLITE3_CANCEL_AUTO_EXTENSION
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).load_extension {
            __SQLITE3_LOAD_EXTENSION
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).malloc64 {
            __SQLITE3_MALLOC64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).msize {
            __SQLITE3_MSIZE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).realloc64 {
            __SQLITE3_REALLOC64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).reset_auto_extension {
            __SQLITE3_RESET_AUTO_EXTENSION
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_blob64 {
            __SQLITE3_RESULT_BLOB64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_text64 {
            __SQLITE3_RESULT_TEXT64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).strglob {
            __SQLITE3_STRGLOB
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_dup {
            __SQLITE3_VALUE_DUP
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_free {
            __SQLITE3_VALUE_FREE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_zeroblob64 {
            __SQLITE3_RESULT_ZEROBLOB64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_zeroblob64 {
            __SQLITE3_BIND_ZEROBLOB64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_subtype {
            __SQLITE3_VALUE_SUBTYPE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_subtype {
            __SQLITE3_RESULT_SUBTYPE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).status64 {
            __SQLITE3_STATUS64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).strlike {
            __SQLITE3_STRLIKE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).db_cacheflush {
            __SQLITE3_DB_CACHEFLUSH
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).system_errno {
            __SQLITE3_SYSTEM_ERRNO
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).trace_v2 {
            __SQLITE3_TRACE_V2
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).expanded_sql {
            __SQLITE3_EXPANDED_SQL
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).set_last_insert_rowid {
            __SQLITE3_SET_LAST_INSERT_ROWID
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).prepare_v3 {
            __SQLITE3_PREPARE_V3
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).bind_pointer {
            __SQLITE3_BIND_POINTER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).result_pointer {
            __SQLITE3_RESULT_POINTER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_pointer {
            __SQLITE3_VALUE_POINTER
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).vtab_nochange {
            __SQLITE3_VTAB_NOCHANGE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_nochange {
            __SQLITE3_VALUE_NOCHANGE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).vtab_collation {
            __SQLITE3_VTAB_COLLATION
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).keyword_count {
            __SQLITE3_KEYWORD_COUNT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).keyword_name {
            __SQLITE3_KEYWORD_NAME
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).keyword_check {
            __SQLITE3_KEYWORD_CHECK
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).str_new {
            __SQLITE3_STR_NEW
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).str_finish {
            __SQLITE3_STR_FINISH
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).str_append {
            __SQLITE3_STR_APPEND
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).str_appendall {
            __SQLITE3_STR_APPENDALL
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).str_appendchar {
            __SQLITE3_STR_APPENDCHAR
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).str_reset {
            __SQLITE3_STR_RESET
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).str_errcode {
            __SQLITE3_STR_ERRCODE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).str_length {
            __SQLITE3_STR_LENGTH
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).str_value {
            __SQLITE3_STR_VALUE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).create_window_function {
            __SQLITE3_CREATE_WINDOW_FUNCTION
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).normalized_sql {
            __SQLITE3_NORMALIZED_SQL
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).stmt_isexplain {
            __SQLITE3_STMT_ISEXPLAIN
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).value_frombind {
            __SQLITE3_VALUE_FROMBIND
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).drop_modules {
            __SQLITE3_DROP_MODULES
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).hard_heap_limit64 {
            __SQLITE3_HARD_HEAP_LIMIT64
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).uri_key {
            __SQLITE3_URI_KEY
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).filename_database {
            __SQLITE3_FILENAME_DATABASE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).filename_journal {
            __SQLITE3_FILENAME_JOURNAL
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).filename_wal {
            __SQLITE3_FILENAME_WAL
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).create_filename {
            __SQLITE3_CREATE_FILENAME
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).free_filename {
            __SQLITE3_FREE_FILENAME
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).database_file_object {
            __SQLITE3_DATABASE_FILE_OBJECT
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        if let Some(fun) = (*p_api).txn_state {
            __SQLITE3_TXN_STATE
                .store(fun as usize as *mut (), crate::sync::atomic::Ordering::Release);
        }
        Ok(())
    }


}
///
pub mod path
{
    pub use std::path::{ * };
}
///
pub mod str
{
    pub use std::str::{ * };
}
///
pub mod sync
{
    pub use std::sync::{ * };
}

#[macro_use] mod error;

#[cfg(not(feature = "loadable_extension"))]
pub mod auto_extension;
#[cfg(feature = "backup")]
pub mod backup;
mod bind;
#[cfg(feature = "blob")]
pub mod blob;
mod busy;
#[cfg(feature = "cache")]
mod cache;
#[cfg(feature = "collation")]
mod collation;
mod column;
pub mod config;
#[cfg(any(feature = "functions", feature = "vtab"))]
mod context;
#[cfg(feature = "functions")]
pub mod functions;
#[cfg(feature = "hooks")]
pub mod hooks;
mod inner_connection;
#[cfg(feature = "limits")]
pub mod limits;
#[cfg(feature = "load_extension")]
mod load_extension_guard;
mod params;
mod pragma;
mod raw_statement;
mod row;
#[cfg(feature = "serialize")]
pub mod serialize;
#[cfg(feature = "session")]
pub mod session;
mod statement;
#[cfg(feature = "trace")]
pub mod trace;
mod transaction;
pub mod types;
#[cfg(feature = "unlock_notify")]
mod unlock_notify;
mod version;
#[cfg(feature = "vtab")]
pub mod vtab;

pub(crate) mod util;

// Actually, only sqlite3_enable_load_extension is disabled (not sqlite3_load_extension)
#[cfg(all(feature = "loadable_extension", feature = "load_extension"))]
compile_error!("feature \"loadable_extension\" and feature \"load_extension\" cannot be enabled at the same time");

// Number of cached prepared statements we'll hold on to.
#[cfg(feature = "cache")]
const STATEMENT_CACHE_DEFAULT_CAPACITY: usize = 16;

/// A macro making it more convenient to pass longer lists of
/// parameters as a `&[&dyn ToSql]`.
///
/// # Example
///
/// ```rust,no_run
/// # use rusqlite::{Result, Connection, params};
///
/// struct Person {
///     name: String,
///     age_in_years: u8,
///     data: Option<Vec<u8>>,
/// }
///
/// fn add_person(conn: &Connection, person: &Person) -> Result<()> {
///     conn.execute(
///         "INSERT INTO person(name, age_in_years, data) VALUES (?1, ?2, ?3)",
///         params![person.name, person.age_in_years, person.data],
///     )?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! params {
    () => {
        &[] as &[&dyn $crate::ToSql]
    };
    ($($param:expr),+ $(,)?) => {
        &[$(&$param as &dyn $crate::ToSql),+] as &[&dyn $crate::ToSql]
    };
}

/// A macro making it more convenient to pass lists of named parameters
/// as a `&[(&str, &dyn ToSql)]`.
///
/// # Example
///
/// ```rust,no_run
/// # use rusqlite::{Result, Connection, named_params};
///
/// struct Person {
///     name: String,
///     age_in_years: u8,
///     data: Option<Vec<u8>>,
/// }
///
/// fn add_person(conn: &Connection, person: &Person) -> Result<()> {
///     conn.execute(
///         "INSERT INTO person (name, age_in_years, data)
///          VALUES (:name, :age, :data)",
///         named_params! {
///             ":name": person.name,
///             ":age": person.age_in_years,
///             ":data": person.data,
///         },
///     )?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! named_params {
    () => {
        &[] as &[(&str, &dyn $crate::ToSql)]
    };
    // Note: It's a lot more work to support this as part of the same macro as
    // `params!`, unfortunately.
    ($($param_name:literal: $param_val:expr),+ $(,)?) => {
        &[$(($param_name, &$param_val as &dyn $crate::ToSql)),+] as &[(&str, &dyn $crate::ToSql)]
    };
}

/// Captured identifiers in SQL
///
/// * only SQLite `$x` / `@x` / `:x` syntax works (Rust `&x` syntax does not
///   work).
/// * `$x.y` expression does not work.
///
/// # Example
///
/// ```rust, no_run
/// # use rusqlite::{prepare_and_bind, Connection, Result, Statement};
///
/// fn misc(db: &Connection) -> Result<Statement> {
///     let name = "Lisa";
///     let age = 8;
///     let smart = true;
///     Ok(prepare_and_bind!(db, "SELECT $name, @age, :smart;"))
/// }
/// ```
#[cfg(feature = "rusqlite-macros")]
#[macro_export]
macro_rules! prepare_and_bind {
    ($conn:expr, $sql:literal) => {{
        let mut stmt = $conn.prepare($sql)?;
        $crate::__bind!(stmt $sql);
        stmt
    }};
}

/// Captured identifiers in SQL
///
/// * only SQLite `$x` / `@x` / `:x` syntax works (Rust `&x` syntax does not
///   work).
/// * `$x.y` expression does not work.
#[cfg(feature = "rusqlite-macros")]
#[macro_export]
macro_rules! prepare_cached_and_bind {
    ($conn:expr, $sql:literal) => {{
        let mut stmt = $conn.prepare_cached($sql)?;
        $crate::__bind!(stmt $sql);
        stmt
    }};
}

/// A typedef of the result returned by many methods.
pub type Result<T, E = Error> = result::Result<T, E>;

/// See the [method documentation](#tymethod.optional).
pub trait OptionalExtension<T> {
    /// Converts a `Result<T>` into a `Result<Option<T>>`.
    ///
    /// By default, Rusqlite treats 0 rows being returned from a query that is
    /// expected to return 1 row as an error. This method will
    /// handle that error, and give you back an `Option<T>` instead.
    fn optional(self) -> Result<Option<T>>;
}

impl<T> OptionalExtension<T> for Result<T> {
    fn optional(self) -> Result<Option<T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

unsafe fn errmsg_to_string(errmsg: *const c_char) -> String {
    CStr::from_ptr(errmsg).to_string_lossy().into_owned()
}

#[cfg(any(feature = "functions", feature = "vtab", test))]
fn str_to_cstring(s: &str) -> Result<util::SmallCString> {
    Ok(util::SmallCString::new(s)?)
}

/// Returns `(string ptr, len as c_int, SQLITE_STATIC | SQLITE_TRANSIENT)`
/// normally.
/// The `sqlite3_destructor_type` item is always `SQLITE_TRANSIENT` unless
/// the string was empty (in which case it's `SQLITE_STATIC`, and the ptr is
/// static).
fn str_for_sqlite(
    s: &[u8],
) -> (
    *const c_char,
    ffi::sqlite3_uint64,
    ffi::sqlite3_destructor_type,
) {
    let len = s.len();
    let (ptr, dtor_info) = if len != 0 {
        (s.as_ptr().cast::<c_char>(), ffi::SQLITE_TRANSIENT())
    } else {
        // Return a pointer guaranteed to live forever
        ("".as_ptr().cast::<c_char>(), ffi::SQLITE_STATIC())
    };
    (ptr, len as ffi::sqlite3_uint64, dtor_info)
}

#[cfg(unix)]
fn path_to_cstring(p: &Path) -> Result<CString> {
    use std::os::unix::ffi::OsStrExt;
    Ok(CString::new(p.as_os_str().as_bytes())?)
}

#[cfg(not(unix))]
fn path_to_cstring(p: &Path) -> Result<CString> {
    let s = p.to_str().ok_or_else(|| Error::InvalidPath(p.to_owned()))?;
    Ok(CString::new(s)?)
}

/// Shorthand for `Main` database.
pub const MAIN_DB: &CStr = c"main";
/// Shorthand for `Temp` database.
pub const TEMP_DB: &CStr = c"temp";

/// A connection to a SQLite database.
pub struct Connection {
    db: RefCell<InnerConnection>,
    #[cfg(feature = "cache")]
    cache: StatementCache,
    transaction_behavior: TransactionBehavior,
}

unsafe impl Send for Connection {}

impl Drop for Connection {
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "cache")]
        self.flush_prepared_statement_cache();
    }
}

impl Connection {
    /// Open a new connection to a SQLite database. If a database does not exist
    /// at the path, one is created.
    ///
    /// ```rust,no_run
    /// # use rusqlite::{Connection, Result};
    /// fn open_my_db() -> Result<()> {
    ///     let path = "./my_db.db3";
    ///     let db = Connection::open(path)?;
    ///     // Use the database somehow...
    ///     println!("{}", db.is_autocommit());
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Flags
    ///
    /// `Connection::open(path)` is equivalent to using
    /// [`Connection::open_with_flags`] with the default [`OpenFlags`]. That is,
    /// it's equivalent to:
    ///
    /// ```ignore
    /// Connection::open_with_flags(
    ///     path,
    ///     OpenFlags::SQLITE_OPEN_READ_WRITE
    ///         | OpenFlags::SQLITE_OPEN_CREATE
    ///         | OpenFlags::SQLITE_OPEN_URI
    ///         | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    /// )
    /// ```
    ///
    /// These flags have the following effects:
    ///
    /// - Open the database for both reading or writing.
    /// - Create the database if one does not exist at the path.
    /// - Allow the filename to be interpreted as a URI (see <https://www.sqlite.org/uri.html#uri_filenames_in_sqlite>
    ///   for details).
    /// - Disables the use of a per-connection mutex.
    ///
    ///   Rusqlite enforces thread-safety at compile time, so additional
    ///   locking is not needed and provides no benefit. (See the
    ///   documentation on [`OpenFlags::SQLITE_OPEN_FULL_MUTEX`] for some
    ///   additional discussion about this).
    ///
    /// Most of these are also the default settings for the C API, although
    /// technically the default locking behavior is controlled by the flags used
    /// when compiling SQLite -- rather than let it vary, we choose `NO_MUTEX`
    /// because it's a fairly clearly the best choice for users of this library.
    ///
    /// # Failure
    ///
    /// Will return `Err` if `path` cannot be converted to a C-compatible string
    /// or if the underlying SQLite open call fails.
    ///
    /// # WASM support
    ///
    /// If you plan to use this connection type on the `wasm32-unknown-unknown` target please
    /// make sure to read the following notes:
    ///
    /// - The database is stored in memory by default.
    /// - Persistent VFS (Virtual File Systems) is optional,
    ///   see <https://github.com/Spxg/sqlite-wasm-rs> for details
    #[inline]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let flags = OpenFlags::default();
        Self::open_with_flags(path, flags)
    }

    /// Open a new connection to an in-memory SQLite database.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the underlying SQLite open call fails.
    #[inline]
    pub fn open_in_memory() -> Result<Self> {
        let flags = OpenFlags::default();
        Self::open_in_memory_with_flags(flags)
    }

    /// Open a new connection to a SQLite database.
    ///
    /// [Database Connection](http://www.sqlite.org/c3ref/open.html) for a description of valid
    /// flag combinations.
    ///
    /// # Failure
    ///
    /// Will return `Err` if `path` cannot be converted to a C-compatible
    /// string or if the underlying SQLite open call fails.
    #[inline]
    pub fn open_with_flags<P: AsRef<Path>>(path: P, flags: OpenFlags) -> Result<Self> {
        let c_path = path_to_cstring(path.as_ref())?;
        InnerConnection::open_with_flags(&c_path, flags, None).map(|db| Self {
            db: RefCell::new(db),
            #[cfg(feature = "cache")]
            cache: StatementCache::with_capacity(STATEMENT_CACHE_DEFAULT_CAPACITY),
            transaction_behavior: TransactionBehavior::Deferred,
        })
    }

    /// Open a new connection to a SQLite database using the specific flags and
    /// vfs name.
    ///
    /// [Database Connection](http://www.sqlite.org/c3ref/open.html) for a description of valid
    /// flag combinations.
    ///
    /// # Failure
    ///
    /// Will return `Err` if either `path` or `vfs` cannot be converted to a
    /// C-compatible string or if the underlying SQLite open call fails.
    #[inline]
    pub fn open_with_flags_and_vfs<P: AsRef<Path>, V: Name>(
        path: P,
        flags: OpenFlags,
        vfs: V,
    ) -> Result<Self> {
        let c_path = path_to_cstring(path.as_ref())?;
        let c_vfs = vfs.as_cstr()?;
        InnerConnection::open_with_flags(&c_path, flags, Some(&c_vfs)).map(|db| Self {
            db: RefCell::new(db),
            #[cfg(feature = "cache")]
            cache: StatementCache::with_capacity(STATEMENT_CACHE_DEFAULT_CAPACITY),
            transaction_behavior: TransactionBehavior::Deferred,
        })
    }

    /// Open a new connection to an in-memory SQLite database.
    ///
    /// [Database Connection](http://www.sqlite.org/c3ref/open.html) for a description of valid
    /// flag combinations.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the underlying SQLite open call fails.
    #[inline]
    pub fn open_in_memory_with_flags(flags: OpenFlags) -> Result<Self> {
        Self::open_with_flags(":memory:", flags)
    }

    /// Open a new connection to an in-memory SQLite database using the specific
    /// flags and vfs name.
    ///
    /// [Database Connection](http://www.sqlite.org/c3ref/open.html) for a description of valid
    /// flag combinations.
    ///
    /// # Failure
    ///
    /// Will return `Err` if `vfs` cannot be converted to a C-compatible
    /// string or if the underlying SQLite open call fails.
    #[inline]
    pub fn open_in_memory_with_flags_and_vfs<V: Name>(flags: OpenFlags, vfs: V) -> Result<Self> {
        Self::open_with_flags_and_vfs(":memory:", flags, vfs)
    }

    /// Convenience method to run multiple SQL statements (that cannot take any
    /// parameters).
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use rusqlite::{Connection, Result};
    /// fn create_tables(conn: &Connection) -> Result<()> {
    ///     conn.execute_batch(
    ///         "BEGIN;
    ///          CREATE TABLE foo(x INTEGER);
    ///          CREATE TABLE bar(y TEXT);
    ///          COMMIT;",
    ///     )
    /// }
    /// ```
    ///
    /// # Failure
    ///
    /// Will return `Err` if `sql` cannot be converted to a C-compatible string
    /// or if the underlying SQLite call fails.
    pub fn execute_batch(&self, sql: &str) -> Result<()> {
        let mut sql = sql;
        while !sql.is_empty() {
            let (stmt, tail) = self
                .db
                .borrow_mut()
                .prepare(self, sql, PrepFlags::default())?;
            if !stmt.stmt.is_null() && stmt.step()? {
                // Some PRAGMA may return rows
                if false {
                    return Err(Error::ExecuteReturnedResults);
                }
            }
            if tail == 0 || tail >= sql.len() {
                break;
            }
            sql = &sql[tail..];
        }
        Ok(())
    }

    /// Convenience method to prepare and execute a single SQL statement.
    ///
    /// On success, returns the number of rows that were changed or inserted or
    /// deleted (via `sqlite3_changes`).
    ///
    /// ## Example
    ///
    /// ### With positional params
    ///
    /// ```rust,no_run
    /// # use rusqlite::{Connection};
    /// fn update_rows(conn: &Connection) {
    ///     match conn.execute("UPDATE foo SET bar = 'baz' WHERE qux = ?1", [1i32]) {
    ///         Ok(updated) => println!("{} rows were updated", updated),
    ///         Err(err) => println!("update failed: {}", err),
    ///     }
    /// }
    /// ```
    ///
    /// ### With positional params of varying types
    ///
    /// ```rust,no_run
    /// # use rusqlite::{params, Connection};
    /// fn update_rows(conn: &Connection) {
    ///     match conn.execute(
    ///         "UPDATE foo SET bar = 'baz' WHERE qux = ?1 AND quux = ?2",
    ///         params![1i32, 1.5f64],
    ///     ) {
    ///         Ok(updated) => println!("{} rows were updated", updated),
    ///         Err(err) => println!("update failed: {}", err),
    ///     }
    /// }
    /// ```
    ///
    /// ### With named params
    ///
    /// ```rust,no_run
    /// # use rusqlite::{Connection, Result};
    /// fn insert(conn: &Connection) -> Result<usize> {
    ///     conn.execute(
    ///         "INSERT INTO test (name) VALUES (:name)",
    ///         &[(":name", "one")],
    ///     )
    /// }
    /// ```
    ///
    /// # Failure
    ///
    /// Will return `Err` if `sql` cannot be converted to a C-compatible string
    /// or if the underlying SQLite call fails.
    #[inline]
    pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize> {
        self.prepare(sql).and_then(|mut stmt| stmt.execute(params))
    }

    /// Returns the path to the database file, if one exists and is known.
    ///
    /// Returns `Some("")` for a temporary or in-memory database.
    ///
    /// Note that in some cases [PRAGMA
    /// database_list](https://sqlite.org/pragma.html#pragma_database_list) is
    /// likely to be more robust.
    #[inline]
    pub fn path(&self) -> Option<&str> {
        unsafe {
            crate::inner_connection::db_filename(std::marker::PhantomData, self.handle(), MAIN_DB)
        }
    }

    /// Attempts to free as much heap memory as possible from the database
    /// connection.
    ///
    /// This calls [`sqlite3_db_release_memory`](https://www.sqlite.org/c3ref/db_release_memory.html).
    #[inline]
    pub fn release_memory(&self) -> Result<()> {
        self.db.borrow_mut().release_memory()
    }

    /// Get the SQLite rowid of the most recent successful INSERT.
    ///
    /// Uses [sqlite3_last_insert_rowid](https://www.sqlite.org/c3ref/last_insert_rowid.html) under
    /// the hood.
    #[inline]
    pub fn last_insert_rowid(&self) -> i64 {
        self.db.borrow_mut().last_insert_rowid()
    }

    /// Convenience method to execute a query that is expected to return a
    /// single row.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use rusqlite::{Result, Connection};
    /// fn preferred_locale(conn: &Connection) -> Result<String> {
    ///     conn.query_row(
    ///         "SELECT value FROM preferences WHERE name='locale'",
    ///         [],
    ///         |row| row.get(0),
    ///     )
    /// }
    /// ```
    ///
    /// If the query returns more than one row, all rows except the first are
    /// ignored.
    ///
    /// Returns `Err(QueryReturnedNoRows)` if no results are returned. If the
    /// query truly is optional, you can call `.optional()` on the result of
    /// this to get a `Result<Option<T>>`.
    ///
    /// # Failure
    ///
    /// Will return `Err` if `sql` cannot be converted to a C-compatible string
    /// or if the underlying SQLite call fails.
    #[inline]
    pub fn query_row<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>
    where
        P: Params,
        F: FnOnce(&Row<'_>) -> Result<T>,
    {
        let mut stmt = self.prepare(sql)?;
        stmt.query_row(params, f)
    }

    /// Convenience method to execute a query that is expected to return exactly
    /// one row.
    ///
    /// Returns `Err(QueryReturnedMoreThanOneRow)` if the query returns more than one row.
    ///
    /// Returns `Err(QueryReturnedNoRows)` if no results are returned. If the
    /// query truly is optional, you can call
    /// [`.optional()`](crate::OptionalExtension::optional) on the result of
    /// this to get a `Result<Option<T>>` (requires that the trait
    /// `rusqlite::OptionalExtension` is imported).
    ///
    /// # Failure
    ///
    /// Will return `Err` if the underlying SQLite call fails.
    pub fn query_one<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>
    where
        P: Params,
        F: FnOnce(&Row<'_>) -> Result<T>,
    {
        let mut stmt = self.prepare(sql)?;
        stmt.query_one(params, f)
    }

    // https://sqlite.org/tclsqlite.html#onecolumn
    #[cfg(test)]
    pub(crate) fn one_column<T, P>(&self, sql: &str, params: P) -> Result<T>
    where
        T: types::FromSql,
        P: Params,
    {
        self.query_one(sql, params, |r| r.get(0))
    }

    /// Convenience method to execute a query that is expected to return a
    /// single row, and execute a mapping via `f` on that returned row with
    /// the possibility of failure. The `Result` type of `f` must implement
    /// `std::convert::From<Error>`.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use rusqlite::{Result, Connection};
    /// fn preferred_locale(conn: &Connection) -> Result<String> {
    ///     conn.query_row_and_then(
    ///         "SELECT value FROM preferences WHERE name='locale'",
    ///         [],
    ///         |row| row.get(0),
    ///     )
    /// }
    /// ```
    ///
    /// If the query returns more than one row, all rows except the first are
    /// ignored.
    ///
    /// # Failure
    ///
    /// Will return `Err` if `sql` cannot be converted to a C-compatible string
    /// or if the underlying SQLite call fails.
    #[inline]
    pub fn query_row_and_then<T, E, P, F>(&self, sql: &str, params: P, f: F) -> Result<T, E>
    where
        P: Params,
        F: FnOnce(&Row<'_>) -> Result<T, E>,
        E: From<Error>,
    {
        let mut stmt = self.prepare(sql)?;
        let mut rows = stmt.query(params)?;

        rows.get_expected_row().map_err(E::from).and_then(f)
    }

    /// Prepare a SQL statement for execution.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use rusqlite::{Connection, Result};
    /// fn insert_new_people(conn: &Connection) -> Result<()> {
    ///     let mut stmt = conn.prepare("INSERT INTO People (name) VALUES (?1)")?;
    ///     stmt.execute(["Joe Smith"])?;
    ///     stmt.execute(["Bob Jones"])?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Failure
    ///
    /// Will return `Err` if `sql` cannot be converted to a C-compatible string
    /// or if the underlying SQLite call fails.
    #[inline]
    pub fn prepare(&self, sql: &str) -> Result<Statement<'_>> {
        self.prepare_with_flags(sql, PrepFlags::default())
    }

    /// Prepare a SQL statement for execution.
    ///
    /// # Failure
    ///
    /// Will return `Err` if `sql` cannot be converted to a C-compatible string
    /// or if the underlying SQLite call fails.
    #[inline]
    pub fn prepare_with_flags(&self, sql: &str, flags: PrepFlags) -> Result<Statement<'_>> {
        let (stmt, tail) = self.db.borrow_mut().prepare(self, sql, flags)?;
        if tail != 0 && !self.prepare(&sql[tail..])?.stmt.is_null() {
            Err(Error::MultipleStatement)
        } else {
            Ok(stmt)
        }
    }

    /// Close the SQLite connection.
    ///
    /// This is functionally equivalent to the `Drop` implementation for
    /// `Connection` except that on failure, it returns an error and the
    /// connection itself (presumably so closing can be attempted again).
    ///
    /// # Failure
    ///
    /// Will return `Err` if the underlying SQLite call fails.
    #[expect(clippy::result_large_err)]
    #[inline]
    pub fn close(self) -> Result<(), (Self, Error)> {
        #[cfg(feature = "cache")]
        self.flush_prepared_statement_cache();
        let r = self.db.borrow_mut().close();
        r.map_err(move |err| (self, err))
    }

    /// Enable loading of SQLite extensions from both SQL queries and Rust.
    ///
    /// You must call [`Connection::load_extension_disable`] when you're
    /// finished loading extensions (failure to call it can lead to bad things,
    /// see "Safety"), so you should strongly consider using
    /// [`LoadExtensionGuard`] instead of this function, automatically disables
    /// extension loading when it goes out of scope.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rusqlite::{Connection, Result};
    /// fn load_my_extension(conn: &Connection) -> Result<()> {
    ///     // Safety: We fully trust the loaded extension and execute no untrusted SQL
    ///     // while extension loading is enabled.
    ///     unsafe {
    ///         conn.load_extension_enable()?;
    ///         let r = conn.load_extension("my/trusted/extension", None::<&str>);
    ///         conn.load_extension_disable()?;
    ///         r
    ///     }
    /// }
    /// ```
    ///
    /// # Failure
    ///
    /// Will return `Err` if the underlying SQLite call fails.
    ///
    /// # Safety
    ///
    /// TLDR: Don't execute any untrusted queries between this call and
    /// [`Connection::load_extension_disable`].
    ///
    /// Perhaps surprisingly, this function does not only allow the use of
    /// [`Connection::load_extension`] from Rust, but it also allows SQL queries
    /// to perform [the same operation][loadext]. For example, in the period
    /// between `load_extension_enable` and `load_extension_disable`, the
    /// following operation will load and call some function in some dynamic
    /// library:
    ///
    /// ```sql
    /// SELECT load_extension('why_is_this_possible.dll', 'dubious_func');
    /// ```
    ///
    /// This means that while this is enabled a carefully crafted SQL query can
    /// be used to escalate a SQL injection attack into code execution.
    ///
    /// Safely using this function requires that you trust all SQL queries run
    /// between when it is called, and when loading is disabled (by
    /// [`Connection::load_extension_disable`]).
    ///
    /// [loadext]: https://www.sqlite.org/lang_corefunc.html#load_extension
    #[cfg(feature = "load_extension")]
    #[inline]
    pub unsafe fn load_extension_enable(&self) -> Result<()> {
        self.db.borrow_mut().enable_load_extension(1)
    }

    /// Disable loading of SQLite extensions.
    ///
    /// See [`Connection::load_extension_enable`] for an example.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the underlying SQLite call fails.
    #[cfg(feature = "load_extension")]
    #[inline]
    pub fn load_extension_disable(&self) -> Result<()> {
        // It's always safe to turn off extension loading.
        unsafe { self.db.borrow_mut().enable_load_extension(0) }
    }

    /// Load the SQLite extension at `dylib_path`. `dylib_path` is passed
    /// through to `sqlite3_load_extension`, which may attempt OS-specific
    /// modifications if the file cannot be loaded directly (for example
    /// converting `"some/ext"` to `"some/ext.so"`, `"some\\ext.dll"`, ...).
    ///
    /// If `entry_point` is `None`, SQLite will attempt to find the entry point.
    /// If it is not `None`, the entry point will be passed through to
    /// `sqlite3_load_extension`.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use rusqlite::{Connection, Result, LoadExtensionGuard};
    /// fn load_my_extension(conn: &Connection) -> Result<()> {
    ///     // Safety: we don't execute any SQL statements while
    ///     // extension loading is enabled.
    ///     let _guard = unsafe { LoadExtensionGuard::new(conn)? };
    ///     // Safety: `my_sqlite_extension` is highly trustworthy.
    ///     unsafe { conn.load_extension("my_sqlite_extension", None::<&str>) }
    /// }
    /// ```
    ///
    /// # Failure
    ///
    /// Will return `Err` if the underlying SQLite call fails.
    ///
    /// # Safety
    ///
    /// This is equivalent to performing a `dlopen`/`LoadLibrary` on a shared
    /// library, and calling a function inside, and thus requires that you trust
    /// the library that you're loading.
    ///
    /// That is to say: to safely use this, the code in the extension must be
    /// sound, trusted, correctly use the SQLite APIs, and not contain any
    /// memory or thread safety errors.
    #[cfg(feature = "load_extension")]
    #[inline]
    pub unsafe fn load_extension<P: AsRef<Path>, N: Name>(
        &self,
        dylib_path: P,
        entry_point: Option<N>,
    ) -> Result<()> {
        self.db
            .borrow_mut()
            .load_extension(dylib_path.as_ref(), entry_point)
    }

    /// Get access to the underlying SQLite database connection handle.
    ///
    /// # Warning
    ///
    /// You should not need to use this function. If you do need to, please
    /// [open an issue on the rusqlite repository](https://github.com/rusqlite/rusqlite/issues) and describe
    /// your use case.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it gives you raw access
    /// to the SQLite connection, and what you do with it could impact the
    /// safety of this `Connection`.
    #[inline]
    pub unsafe fn handle(&self) -> *mut ffi::sqlite3 {
        self.db.borrow().db()
    }

    /// Create a `Connection` from a raw handle.
    ///
    /// The underlying SQLite database connection handle will not be closed when
    /// the returned connection is dropped/closed.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may impact the Connection.
    #[inline]
    pub unsafe fn from_handle(db: *mut ffi::sqlite3) -> Result<Self> {
        let db = InnerConnection::new(db, false);
        Ok(Self {
            db: RefCell::new(db),
            #[cfg(feature = "cache")]
            cache: StatementCache::with_capacity(STATEMENT_CACHE_DEFAULT_CAPACITY),
            transaction_behavior: TransactionBehavior::Deferred,
        })
    }

    /// Helper to register an SQLite extension written in Rust.
    /// For [persistent](https://sqlite.org/loadext.html#persistent_loadable_extensions) extension,
    /// `init` should return `Ok(true)`.
    /// # Safety
    /// * Results are undefined if `init` does not just register features.
    #[cfg(feature = "loadable_extension")]
    pub unsafe fn extension_init2(
        db: *mut ffi::sqlite3,
        pz_err_msg: *mut *mut c_char,
        p_api: *mut ffi::sqlite3_api_routines,
        init: fn(Self) -> Result<bool>,
    ) -> c_int {
        if p_api.is_null() {
            return ffi::SQLITE_ERROR;
        }
        match ffi::rusqlite_extension_init2(p_api)
            .map_err(Error::from)
            .and(Self::from_handle(db))
            .and_then(init)
        {
            Err(err) => to_sqlite_error(&err, pz_err_msg),
            Ok(true) => ffi::SQLITE_OK_LOAD_PERMANENTLY,
            _ => ffi::SQLITE_OK,
        }
    }

    /// Create a `Connection` from a raw owned handle.
    ///
    /// The returned connection will attempt to close the inner connection
    /// when dropped/closed. This function should only be called on connections
    /// owned by the caller.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may impact the Connection.
    /// In particular, it should only be called on connections created
    /// and owned by the caller, e.g. as a result of calling
    /// `ffi::sqlite3_open`().
    #[inline]
    pub unsafe fn from_handle_owned(db: *mut ffi::sqlite3) -> Result<Self> {
        let db = InnerConnection::new(db, true);
        Ok(Self {
            db: RefCell::new(db),
            #[cfg(feature = "cache")]
            cache: StatementCache::with_capacity(STATEMENT_CACHE_DEFAULT_CAPACITY),
            transaction_behavior: TransactionBehavior::Deferred,
        })
    }

    /// Get access to a handle that can be used to interrupt long-running
    /// queries from another thread.
    #[inline]
    pub fn get_interrupt_handle(&self) -> InterruptHandle {
        self.db.borrow().get_interrupt_handle()
    }

    #[inline]
    fn decode_result(&self, code: c_int) -> Result<()> {
        self.db.borrow().decode_result(code)
    }

    /// Return the number of rows modified, inserted or deleted by the most
    /// recently completed INSERT, UPDATE or DELETE statement on the database
    /// connection.
    ///
    /// See <https://www.sqlite.org/c3ref/changes.html>
    #[inline]
    pub fn changes(&self) -> u64 {
        self.db.borrow().changes()
    }

    /// Return the total number of rows modified, inserted or deleted by all
    /// completed INSERT, UPDATE or DELETE statements since the database
    /// connection was opened, including those executed as part of trigger programs.
    ///
    /// See <https://www.sqlite.org/c3ref/total_changes.html>
    #[inline]
    pub fn total_changes(&self) -> u64 {
        self.db.borrow().total_changes()
    }

    /// Test for auto-commit mode.
    /// Autocommit mode is on by default.
    #[inline]
    pub fn is_autocommit(&self) -> bool {
        self.db.borrow().is_autocommit()
    }

    /// Determine if all associated prepared statements have been reset.
    #[inline]
    pub fn is_busy(&self) -> bool {
        self.db.borrow().is_busy()
    }

    /// Flush caches to disk mid-transaction
    pub fn cache_flush(&self) -> Result<()> {
        self.db.borrow_mut().cache_flush()
    }

    /// Determine if a database is read-only
    pub fn is_readonly<N: Name>(&self, db_name: N) -> Result<bool> {
        self.db.borrow().db_readonly(db_name)
    }

    /// Return the schema name for a database connection
    ///
    /// ## Failure
    ///
    /// Return an `Error::InvalidDatabaseIndex` if `index` is out of range.
    #[cfg(feature = "modern_sqlite")] // 3.39.0
    pub fn db_name(&self, index: usize) -> Result<String> {
        unsafe {
            let db = self.handle();
            let name = ffi::sqlite3_db_name(db, index as c_int);
            if name.is_null() {
                Err(Error::InvalidDatabaseIndex(index))
            } else {
                Ok(CStr::from_ptr(name).to_str()?.to_owned())
            }
        }
    }

    /// Determine whether an interrupt is currently in effect
    #[cfg(feature = "modern_sqlite")] // 3.41.0
    pub fn is_interrupted(&self) -> bool {
        self.db.borrow().is_interrupted()
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection")
            .field("path", &self.path())
            .finish()
    }
}

/// Batch fallible iterator
///
/// # Warning
///
/// There is no recovery on parsing error, when a invalid statement is found in `sql`, SQLite cannot jump to the next statement.
/// So you should break the loop when an error is raised by the `next` method.
///
/// ```rust
/// use fallible_iterator::FallibleIterator;
/// use rusqlite::{Batch, Connection, Result};
///
/// fn main() -> Result<()> {
///     let conn = Connection::open_in_memory()?;
///     let sql = r"
///     CREATE TABLE tbl1 (col);
///     CREATE TABLE tbl2 (col);
///     ";
///     let mut batch = Batch::new(&conn, sql);
///     while let Some(mut stmt) = batch.next()? {
///         stmt.execute([])?;
///     }
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Batch<'conn, 'sql> {
    conn: &'conn Connection,
    sql: &'sql str,
    tail: usize,
}

impl<'conn, 'sql> Batch<'conn, 'sql> {
    /// Constructor
    pub fn new(conn: &'conn Connection, sql: &'sql str) -> Self {
        Batch { conn, sql, tail: 0 }
    }
}
impl<'conn> fallible_iterator::FallibleIterator for Batch<'conn, '_> {
    type Error = Error;
    type Item = Statement<'conn>;

    /// Iterates on each batch statements.
    ///
    /// Returns `Ok(None)` when batch is completed.
    fn next(&mut self) -> Result<Option<Statement<'conn>>> {
        while self.tail < self.sql.len() {
            let sql = &self.sql[self.tail..];
            let (next, tail) =
                self.conn
                    .db
                    .borrow_mut()
                    .prepare(self.conn, sql, PrepFlags::default())?;
            if tail == 0 {
                self.tail = self.sql.len();
            } else {
                self.tail += tail;
            }
            if next.stmt.is_null() {
                continue;
            }
            return Ok(Some(next));
        }
        Ok(None)
    }
}

bitflags::bitflags! {
    /// Flags for opening SQLite database connections. See
    /// [sqlite3_open_v2](https://www.sqlite.org/c3ref/open.html) for details.
    ///
    /// The default open flags are `SQLITE_OPEN_READ_WRITE | SQLITE_OPEN_CREATE
    /// | SQLITE_OPEN_URI | SQLITE_OPEN_NO_MUTEX`. See [`Connection::open`] for
    /// some discussion about these flags.
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[repr(C)]
    pub struct OpenFlags: c_int {
        /// The database is opened in read-only mode.
        /// If the database does not already exist, an error is returned.
        const SQLITE_OPEN_READ_ONLY = ffi::SQLITE_OPEN_READONLY;
        /// The database is opened for reading and writing if possible,
        /// or reading only if the file is write-protected by the operating system.
        /// In either case the database must already exist, otherwise an error is returned.
        const SQLITE_OPEN_READ_WRITE = ffi::SQLITE_OPEN_READWRITE;
        /// The database is created if it does not already exist
        const SQLITE_OPEN_CREATE = ffi::SQLITE_OPEN_CREATE;
        /// The filename can be interpreted as a URI if this flag is set.
        const SQLITE_OPEN_URI = ffi::SQLITE_OPEN_URI;
        /// The database will be opened as an in-memory database.
        const SQLITE_OPEN_MEMORY = ffi::SQLITE_OPEN_MEMORY;
        /// The new database connection will not use a per-connection mutex (the
        /// connection will use the "multi-thread" threading mode, in SQLite
        /// parlance).
        ///
        /// This is used by default, as proper `Send`/`Sync` usage (in
        /// particular, the fact that [`Connection`] does not implement `Sync`)
        /// ensures thread-safety without the need to perform locking around all
        /// calls.
        const SQLITE_OPEN_NO_MUTEX = ffi::SQLITE_OPEN_NOMUTEX;
        /// The new database connection will use a per-connection mutex -- the
        /// "serialized" threading mode, in SQLite parlance.
        ///
        /// # Caveats
        ///
        /// This flag should probably never be used with `rusqlite`, as we
        /// ensure thread-safety statically (we implement [`Send`] and not
        /// [`Sync`]).
        ///
        /// Critically, even if this flag is used, the [`Connection`] is not
        /// safe to use across multiple threads simultaneously. To access a
        /// database from multiple threads, you should either create multiple
        /// connections, one for each thread (if you have very many threads,
        /// wrapping the `rusqlite::Connection` in a mutex is also reasonable).
        ///
        /// This is both because of the additional per-connection state stored
        /// by `rusqlite` (for example, the prepared statement cache), and
        /// because not all of SQLites functions are fully thread safe, even in
        /// serialized/`SQLITE_OPEN_FULLMUTEX` mode.
        ///
        /// All that said, it's fairly harmless to enable this flag with
        /// `rusqlite`, it will just slow things down while providing no
        /// benefit.
        const SQLITE_OPEN_FULL_MUTEX = ffi::SQLITE_OPEN_FULLMUTEX;
        /// The database is opened with shared cache enabled.
        ///
        /// This is frequently useful for in-memory connections, but note that
        /// broadly speaking it's discouraged by SQLite itself, which states
        /// "Any use of shared cache is discouraged" in the official
        /// [documentation](https://www.sqlite.org/c3ref/enable_shared_cache.html).
        const SQLITE_OPEN_SHARED_CACHE = 0x0002_0000;
        /// The database is opened shared cache disabled.
        const SQLITE_OPEN_PRIVATE_CACHE = 0x0004_0000;
        /// The database filename is not allowed to be a symbolic link. (3.31.0)
        const SQLITE_OPEN_NOFOLLOW = 0x0100_0000;
        /// Extended result codes. (3.37.0)
        const SQLITE_OPEN_EXRESCODE = 0x0200_0000;
    }
}

impl Default for OpenFlags {
    #[inline]
    fn default() -> Self {
        // Note: update the `Connection::open` and top-level `OpenFlags` docs if
        // you change these.
        Self::SQLITE_OPEN_READ_WRITE
            | Self::SQLITE_OPEN_CREATE
            | Self::SQLITE_OPEN_NO_MUTEX
            | Self::SQLITE_OPEN_URI
    }
}

bitflags::bitflags! {
    /// Prepare flags. See
    /// [sqlite3_prepare_v3](https://sqlite.org/c3ref/c_prepare_normalize.html) for details.
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    #[repr(C)]
    pub struct PrepFlags: c_uint {
        /// A hint to the query planner that the prepared statement will be retained for a long time and probably reused many times.
        const SQLITE_PREPARE_PERSISTENT = 0x01;
        /// Causes the SQL compiler to return an error (error code SQLITE_ERROR) if the statement uses any virtual tables.
        const SQLITE_PREPARE_NO_VTAB = 0x04;
        /// Prevents SQL compiler errors from being sent to the error log.
        const SQLITE_PREPARE_DONT_LOG = 0x10;
    }
}

/// Allows interrupting a long-running computation.
pub struct InterruptHandle {
    db_lock: Arc<Mutex<*mut ffi::sqlite3>>,
}

unsafe impl Send for InterruptHandle {}
unsafe impl Sync for InterruptHandle {}

impl InterruptHandle {
    /// Interrupt the query currently executing on another thread. This will
    /// cause that query to fail with a `SQLITE3_INTERRUPT` error.
    pub fn interrupt(&self) {
        let db_handle = self.db_lock.lock().unwrap();
        if !db_handle.is_null() {
            unsafe { ffi::sqlite3_interrupt(*db_handle) }
        }
    }
}

#[cfg(doctest)]
doc_comment::doctest!("../README.md");