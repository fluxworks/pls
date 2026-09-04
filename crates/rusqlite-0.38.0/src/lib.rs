//! Rusqlite is an ergonomic wrapper for using SQLite from Rust.

/*
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use fallible_iterator;
pub use fallible_streaming_iterator;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub use libsqlite3_sys as ffi;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub use sqlite_wasm_rs as ffi;

use std::cell::RefCell;
use std::default::Default;
use std::ffi::{c_char, c_int, c_uint, CStr, CString};
use std::fmt;

use std::path::Path;
use std::result;
use std::str;
use std::sync::{Arc, Mutex};

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
pub use crate::ffi::ErrorCode;
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

#[macro_use]
mod error;

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
*/
pub mod cell
{
    pub use std::cell::{ * };
}

pub mod database
{
    use crate::
    {
        *,
    };

    pub struct Database
    {

    }

    impl Database
    {

    }
}

pub mod connection
{
    use crate::
    {
        cell::{ RefCell },
        database::{ Database },
        sync::{Arc, Mutex},
        *,
    };

    pub struct InnerConnection
    {
        pub db: *mut Database,
        interrupt_lock: Arc<Mutex<*mut Database>>,
        pub commit_hook: Option<Box<dyn FnMut() -> bool + Send>>,
        pub rollback_hook: Option<Box<dyn FnMut() + Send>>,
        pub update_hook: Option<Box<dyn FnMut(crate::hooks::Action, &str, &str, i64) + Send>>,
        pub progress_handler: Option<Box<dyn FnMut() -> bool + Send>>,
        pub authorizer: Option<crate::hooks::BoxedAuthorizer>,
        pub preupdate_hook: Option<Box<dyn FnMut(crate::hooks::Action, &str, &str, &crate::hooks::PreUpdateCase) + Send>,>,
        owned: bool,
    }

    pub struct Connection
    {
        db: RefCell<InnerConnection>,
        cache: StatementCache,
        transaction_behavior: TransactionBehavior,
    }
}

pub mod hooks
{
    use crate::
    {
        *,
    };
}

pub mod statement
{
    use std::cell::RefCell;
    use std::sync::Arc;
    use crate::
    {
        *,
    };

    /// Prepared statements LRU cache.
    #[derive(Debug)]
    pub struct StatementCache(RefCell<LruCache<Arc<str>, RawStatement>>);

    unsafe impl Send for StatementCache {}

    pub mod raw
    {
        use crate::
        {
            *,
        };
    }
}

pub mod hash
{
    pub use std::hash::{ * };
    /*
    hashlink v0.12.1
    hashbrown v0.15.5
    foldhash v0.2.0 */
    #[inline(always)]
    const fn folded_multiply(x: u64, y: u64) -> u64 {
        // The following code path is only fast if 64-bit to 128-bit widening
        // multiplication is supported by the architecture. Most 64-bit
        // architectures except SPARC64 and Wasm64 support it. However, the target
        // pointer width doesn't always indicate that we are dealing with a 64-bit
        // architecture, as there are ABIs that reduce the pointer width, especially
        // on AArch64 and x86-64. WebAssembly (regardless of pointer width) supports
        // 64-bit to 128-bit widening multiplication with the `wide-arithmetic`
        // proposal.
        #[cfg(any(
            all(
                target_pointer_width = "64",
                not(any(target_arch = "sparc64", target_arch = "wasm64")),
            ),
            target_arch = "aarch64",
            target_arch = "x86_64",
            all(target_family = "wasm", target_feature = "wide-arithmetic"),
        ))]
        {
            // We compute the full u64 x u64 -> u128 product, this is a single mul
            // instruction on x86-64, one mul plus one mulhi on ARM64.
            let full = (x as u128).wrapping_mul(y as u128);
            let lo = full as u64;
            let hi = (full >> 64) as u64;

            // The middle bits of the full product fluctuate the most with small
            // changes in the input. This is the top bits of lo and the bottom bits
            // of hi. We can thus make the entire output fluctuate with small
            // changes to the input by XOR'ing these two halves.
            lo ^ hi
        }

        #[cfg(not(any(
            all(
                target_pointer_width = "64",
                not(any(target_arch = "sparc64", target_arch = "wasm64")),
            ),
            target_arch = "aarch64",
            target_arch = "x86_64",
            all(target_family = "wasm", target_feature = "wide-arithmetic"),
        )))]
        {
            // u64 x u64 -> u128 product is quite expensive on 32-bit.
            // We approximate it by expanding the multiplication and eliminating
            // carries by replacing additions with XORs:
            //    (2^32 hx + lx)*(2^32 hy + ly) =
            //    2^64 hx*hy + 2^32 (hx*ly + lx*hy) + lx*ly ~=
            //    2^64 hx*hy ^ 2^32 (hx*ly ^ lx*hy) ^ lx*ly
            // Which when folded becomes:
            //    (hx*hy ^ lx*ly) ^ (hx*ly ^ lx*hy).rotate_right(32)

            let lx = x as u32;
            let ly = y as u32;
            let hx = (x >> 32) as u32;
            let hy = (y >> 32) as u32;

            let ll = (lx as u64).wrapping_mul(ly as u64);
            let lh = (lx as u64).wrapping_mul(hy as u64);
            let hl = (hx as u64).wrapping_mul(ly as u64);
            let hh = (hx as u64).wrapping_mul(hy as u64);

            (hh ^ ll) ^ (hl ^ lh).rotate_right(32)
        }
    }

    #[inline(always)]
    const fn rotate_right(x: u64, r: u32) -> u64 {
        #[cfg(any(
            target_pointer_width = "64",
            target_arch = "aarch64",
            target_arch = "x86_64",
            target_family = "wasm",
        ))]
        {
            x.rotate_right(r)
        }

        #[cfg(not(any(
            target_pointer_width = "64",
            target_arch = "aarch64",
            target_arch = "x86_64",
            target_family = "wasm",
        )))]
        {
            // On platforms without 64-bit arithmetic rotation can be slow, rotate
            // each 32-bit half independently.
            let lo = (x as u32).rotate_right(r);
            let hi = ((x >> 32) as u32).rotate_right(r);
            ((hi as u64) << 32) | lo as u64
        }
    }

    #[cold]
    fn cold_path() {}

    /// Hashes strings <= 16 bytes, has unspecified behavior when bytes.len() > 16.
    #[inline(always)]
    fn hash_bytes_short(bytes: &[u8], accumulator: u64, seeds: &[u64; 6]) -> u64 {
        let len = bytes.len();
        let mut s0 = accumulator;
        let mut s1 = seeds[1];
        // XOR the input into s0, s1, then multiply and fold.
        if len >= 8 {
            s0 ^= u64::from_ne_bytes(bytes[0..8].try_into().unwrap());
            s1 ^= u64::from_ne_bytes(bytes[len - 8..].try_into().unwrap());
        } else if len >= 4 {
            s0 ^= u32::from_ne_bytes(bytes[0..4].try_into().unwrap()) as u64;
            s1 ^= u32::from_ne_bytes(bytes[len - 4..].try_into().unwrap()) as u64;
        } else if len > 0 {
            let lo = bytes[0];
            let mid = bytes[len / 2];
            let hi = bytes[len - 1];
            s0 ^= lo as u64;
            s1 ^= ((hi as u64) << 8) | mid as u64;
        }
        folded_multiply(s0, s1)
    }

    /// Load 8 bytes into a u64 word at the given offset.
    ///
    /// # Safety
    /// You must ensure that offset + 8 <= bytes.len().
    #[inline(always)]
    unsafe fn load(bytes: &[u8], offset: usize) -> u64 {
        unsafe { bytes.as_ptr().add(offset).cast::<u64>().read_unaligned() }
    }

    /// Hashes strings > 16 bytes.
    ///
    /// # Safety
    /// v.len() must be > 16 bytes.
    #[cold]
    #[inline(never)]
    unsafe fn hash_bytes_long(mut v: &[u8], accumulator: u64, seeds: &[u64; 6]) -> u64 {
        let mut s0 = accumulator;
        let mut s1 = s0.wrapping_add(seeds[1]);

        if v.len() > 128 {
            cold_path();
            let mut s2 = s0.wrapping_add(seeds[2]);
            let mut s3 = s0.wrapping_add(seeds[3]);

            if v.len() > 256 {
                cold_path();
                let mut s4 = s0.wrapping_add(seeds[4]);
                let mut s5 = s0.wrapping_add(seeds[5]);
                loop {
                    unsafe {
                        // SAFETY: we checked the length is > 256, we index at most v[..96].
                        s0 = folded_multiply(load(v, 0) ^ s0, load(v, 48) ^ seeds[0]);
                        s1 = folded_multiply(load(v, 8) ^ s1, load(v, 56) ^ seeds[0]);
                        s2 = folded_multiply(load(v, 16) ^ s2, load(v, 64) ^ seeds[0]);
                        s3 = folded_multiply(load(v, 24) ^ s3, load(v, 72) ^ seeds[0]);
                        s4 = folded_multiply(load(v, 32) ^ s4, load(v, 80) ^ seeds[0]);
                        s5 = folded_multiply(load(v, 40) ^ s5, load(v, 88) ^ seeds[0]);
                    }
                    v = &v[96..];
                    if v.len() <= 256 {
                        break;
                    }
                }
                s0 ^= s4;
                s1 ^= s5;
            }

            loop {
                unsafe {
                    s0 = folded_multiply(load(v, 0) ^ s0, load(v, 32) ^ seeds[0]);
                    s1 = folded_multiply(load(v, 8) ^ s1, load(v, 40) ^ seeds[0]);
                    s2 = folded_multiply(load(v, 16) ^ s2, load(v, 48) ^ seeds[0]);
                    s3 = folded_multiply(load(v, 24) ^ s3, load(v, 56) ^ seeds[0]);
                }
                v = &v[64..];
                if v.len() <= 128 {
                    break;
                }
            }
            s0 ^= s2;
            s1 ^= s3;
        }

        let len = v.len();

        unsafe
        {
            s0 = folded_multiply(load(v, 0) ^ s0, load(v, len - 16) ^ seeds[0]);
            s1 = folded_multiply(load(v, 8) ^ s1, load(v, len - 8) ^ seeds[0]);
            if len >= 32 {
                s0 = folded_multiply(load(v, 16) ^ s0, load(v, len - 32) ^ seeds[0]);
                s1 = folded_multiply(load(v, 24) ^ s1, load(v, len - 24) ^ seeds[0]);
                if len >= 64 {
                    s0 = folded_multiply(load(v, 32) ^ s0, load(v, len - 48) ^ seeds[0]);
                    s1 = folded_multiply(load(v, 40) ^ s1, load(v, len - 40) ^ seeds[0]);
                    if len >= 96 {
                        s0 = folded_multiply(load(v, 48) ^ s0, load(v, len - 64) ^ seeds[0]);
                        s1 = folded_multiply(load(v, 56) ^ s1, load(v, len - 56) ^ seeds[0]);
                    }
                }
            }
        }

        s0 ^ s1
    }
    /// A random seed intended to be shared by many different foldhash instances.
    #[derive(Clone, Debug)]
    pub struct SharedSeed
    {
        pub seeds: [u64; 6],
    }
    /*
    hashbrown::DefaultHashBuilder::FoldHasher */
    /// A [`Hasher`] instance implementing foldhash, optimized for speed.
    pub struct DefaultHasher<'a>
    {
        accumulator: u64,
        sponge: u128,
        sponge_len: u8,
        seeds: &'a [u64; 6],
    }

    impl<'a> DefaultHasher<'a>
    {
        /// Initializes this [`FoldHasher`] with the given per-hasher seed and [`SharedSeed`].
        #[inline] pub const fn with_seed(per_hasher_seed: u64, shared_seed: &'a SharedSeed) -> DefaultHasher<'a>
        {
            DefaultHasher
            {
                accumulator: per_hasher_seed,
                sponge: 0,
                sponge_len: 0,
                seeds: &shared_seed.seeds,
            }
        }

        #[inline(always)]
        fn write_num<T: Into<u128>>(&mut self, x: T) {
            let bits: usize = 8 * core::mem::size_of::<T>();
            if self.sponge_len as usize + bits > 128 {
                let lo = self.sponge as u64;
                let hi = (self.sponge >> 64) as u64;
                self.accumulator = folded_multiply(lo ^ self.accumulator, hi ^ self.seeds[0]);
                self.sponge = x.into();
                self.sponge_len = bits as u8;
            } else {
                self.sponge |= x.into() << self.sponge_len;
                self.sponge_len += bits as u8;
            }
        }
    }

    impl<'a> Hasher for DefaultHasher<'a> {
        #[inline(always)]
        fn write(&mut self, bytes: &[u8]) {
            // We perform overlapping reads in the byte hash which could lead to
            // trivial length-extension attacks. These should be defeated by
            // adding a length-dependent rotation on our unpredictable seed
            // which costs only a single cycle (or none if executed with
            // instruction-level parallelism).
            let len = bytes.len();
            self.accumulator = rotate_right(self.accumulator, len as u32);
            if len <= 16 {
                self.accumulator = hash_bytes_short(bytes, self.accumulator, self.seeds);
            } else {
                unsafe {
                    // SAFETY: we checked that the length is > 16 bytes.
                    self.accumulator = hash_bytes_long(bytes, self.accumulator, self.seeds);
                }
            }
        }

        #[inline(always)]
        fn write_u8(&mut self, i: u8) {
            self.write_num(i);
        }

        #[inline(always)]
        fn write_u16(&mut self, i: u16) {
            self.write_num(i);
        }

        #[inline(always)]
        fn write_u32(&mut self, i: u32) {
            self.write_num(i);
        }

        #[inline(always)]
        fn write_u64(&mut self, i: u64) {
            self.write_num(i);
        }

        #[inline(always)]
        fn write_u128(&mut self, i: u128) {
            let lo = i as u64;
            let hi = (i >> 64) as u64;
            self.accumulator = folded_multiply(lo ^ self.accumulator, hi ^ self.seeds[0]);
        }

        #[inline(always)]
        fn write_usize(&mut self, i: usize) {
            // u128 doesn't implement From<usize>.
            #[cfg(target_pointer_width = "32")]
            self.write_num(i as u32);
            #[cfg(target_pointer_width = "64")]
            self.write_num(i as u64);
        }

        #[cfg(feature = "nightly")]
        #[inline(always)]
        fn write_str(&mut self, s: &str) {
            // Our write function already handles length differences.
            self.write(s.as_bytes())
        }

        #[inline(always)]
        fn finish(&self) -> u64 {
            if self.sponge_len > 0 {
                let lo = self.sponge as u64;
                let hi = (self.sponge >> 64) as u64;
                folded_multiply(lo ^ self.accumulator, hi ^ self.seeds[0])
            } else {
                self.accumulator
            }
        }
    }

    /// An object representing an initialized global seed.
    ///
    /// Does not actually store the seed inside itself, it is a zero-sized type.
    /// This prevents inflating the RandomState size and in turn HashMap's size.
    #[derive(Copy, Clone, Debug)]
    pub struct GlobalSeed {
        // So we can't accidentally type GlobalSeed { } within this crate.
        _no_accidental_unsafe_init: (),
    }

    impl GlobalSeed {
        #[inline(always)]
        pub fn new() -> Self {
            if GLOBAL_SEED_STORAGE.state.load(Ordering::Acquire) != INIT {
                Self::init_slow()
            }
            Self {
                _no_accidental_unsafe_init: (),
            }
        }

        #[cold]
        #[inline(never)]
        fn init_slow() {
            // Generate seed outside of critical section.
            let seed = generate_global_seed();

            loop {
                match GLOBAL_SEED_STORAGE.state.compare_exchange_weak(
                    UNINIT,
                    LOCKED,
                    Ordering::Acquire,
                    Ordering::Acquire,
                ) {
                    Ok(_) => unsafe {
                        // SAFETY: we just acquired an exclusive lock.
                        *GLOBAL_SEED_STORAGE.seed.get() = seed;
                        GLOBAL_SEED_STORAGE.state.store(INIT, Ordering::Release);
                        return;
                    },

                    Err(INIT) => return,

                    // Yes, it's a spin loop. We need to support no_std (so no easy
                    // access to proper locks), this is a one-time-per-program
                    // initialization, and the critical section is only a few
                    // store instructions, so it'll be fine.
                    _ => core::hint::spin_loop(),
                }
            }
        }

        #[inline(always)]
        pub fn get(self) -> &'static SharedSeed {
            // SAFETY: our constructor ensured we are in the INIT state and thus
            // this raw read does not race with any write.
            unsafe { &*GLOBAL_SEED_STORAGE.seed.get() }
        }
    }
}

    /// A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly initialized.
    #[derive(Clone, Debug)]
    pub struct RandomState {
        per_hasher_seed: u64,
        global_seed: GlobalSeed,
    }

    impl Default for RandomState {
        #[inline(always)]
        fn default() -> Self {
            Self {
                per_hasher_seed: gen_per_hasher_seed(),
                global_seed: GlobalSeed::new(),
            }
        }
    }

    impl BuildHasher for RandomState {
        type Hasher = FoldHasher<'static>;

        #[inline(always)]
        fn build_hasher(&self) -> FoldHasher<'static> {
            FoldHasher::with_seed(self.per_hasher_seed, self.global_seed.get())
        }
    }

    /// A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly
    /// initialized by default, but can also be initialized with a specific seed.
    ///
    /// This can be useful for e.g. testing, but the downside is that this type
    /// has a size of 16 bytes rather than the 8 bytes [`RandomState`] is.
    #[derive(Clone, Debug)]
    pub struct SeedableRandomState {
        per_hasher_seed: u64,
        shared_seed: &'static SharedSeed,
    }

    impl Default for SeedableRandomState {
        #[inline(always)]
        fn default() -> Self {
            Self::random()
        }
    }

    impl SeedableRandomState {
        /// Generates a random [`SeedableRandomState`], similar to [`RandomState`].
        #[inline(always)]
        pub fn random() -> Self {
            Self {
                per_hasher_seed: gen_per_hasher_seed(),
                shared_seed: SharedSeed::global_random(),
            }
        }

        /// Generates a fixed [`SeedableRandomState`], similar to [`FixedState`].
        #[inline(always)]
        pub fn fixed() -> Self {
            Self {
                per_hasher_seed: ARBITRARY3,
                shared_seed: SharedSeed::global_fixed(),
            }
        }

        /// Generates a [`SeedableRandomState`] with the given per-hasher seed
        /// and [`SharedSeed`].
        #[inline(always)]
        pub fn with_seed(per_hasher_seed: u64, shared_seed: &'static SharedSeed) -> Self {
            // XOR with ARBITRARY3 such that with_seed(0) matches default.
            Self {
                per_hasher_seed: per_hasher_seed ^ ARBITRARY3,
                shared_seed,
            }
        }
    }

    impl BuildHasher for SeedableRandomState {
        type Hasher = FoldHasher<'static>;

        #[inline(always)]
        fn build_hasher(&self) -> FoldHasher<'static> {
            FoldHasher::with_seed(self.per_hasher_seed, self.shared_seed)
        }
    }

    /// A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that always has the same fixed seed.
    ///
    /// Not recommended unless you absolutely need determinism.
    #[derive(Clone, Debug)]
    pub struct FixedState {
        per_hasher_seed: u64,
    }

    impl FixedState {
        /// Creates a [`FixedState`] with the given per-hasher-seed.
        #[inline(always)]
        pub const fn with_seed(per_hasher_seed: u64) -> Self {
            // XOR with ARBITRARY3 such that with_seed(0) matches default.
            Self {
                per_hasher_seed: per_hasher_seed ^ ARBITRARY3,
            }
        }
    }

    impl Default for FixedState {
        #[inline(always)]
        fn default() -> Self {
            Self {
                per_hasher_seed: ARBITRARY3,
            }
        }
    }

    impl BuildHasher for FixedState {
        type Hasher = FoldHasher<'static>;

        #[inline(always)]
        fn build_hasher(&self) -> FoldHasher<'static> {
            FoldHasher::with_seed(self.per_hasher_seed, SharedSeed::global_fixed())
        }
    }

    /// Default hash builder, matches hashbrown's default hasher.
    #[derive(Clone, Default, Debug)]
    pub struct DefaultHashBuilder(DefaultHasher);
}

pub mod lru
{
    use crate::
    {
        *,
    };

    /// A version of `HashMap` that has a user controllable order for its entries.
    pub struct LinkedHashMap<K, V, S = DefaultHashBuilder> {
        table: HashTable<NonNull<Node<K, V>>>,
        hash_builder: S,
        values: Option<NonNull<Node<K, V>>>,
        free: Option<NonNull<Node<K, V>>>,
    }

    pub struct LruCache<K, V, S = DefaultHashBuilder> {
        map: LinkedHashMap<K, V, S>,
        max_size: usize,
    }

}

pub mod sync
{
    pub mod atomic
    {
        pub use std::sync::atomic::{ * };
    }

    pub use std::sync::{ * };
}