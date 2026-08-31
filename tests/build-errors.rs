/*
C:/Users/CKGuest/.cargo/bin/cargo.exe check --color=always --message-format json-diagnostic-rendered-ansi --all-targets --workspace --keep-going
    Checking unicode-width v0.1.14
    Checking winapi-x86_64-pc-windows-gnu v0.4.0
    Checking tinyvec_macros v0.1.1
    Checking bitflags v2.11.1
    Checking smallvec v1.15.1
    Checking memchr v2.8.0
    Checking cfg-if v1.0.4
    Checking foldhash v0.2.0
    Checking textwrap v0.11.0
    Checking winapi v0.3.9
    Checking aho-corasick v1.1.4
    Checking smallstr v0.2.0
    Checking tinyvec v1.11.0
   Compiling libsqlite3-sys v0.36.0
    Checking libc v0.2.185
    Checking hashbrown v0.16.1
    Checking powerfmt v0.2.0
    Checking clap_lex v1.1.0
    Checking regex-syntax v0.8.10
    Checking unicode-normalization v0.1.25
    Checking bitflags v1.3.2
    Checking windows-link v0.2.1
    Checking anstyle v1.0.14
    Checking deranged v0.5.8
    Checking mortal v0.2.4
    Checking hashlink v0.11.0
    Checking clap v2.34.0
    Checking windows-sys v0.61.2
    Checking clap_builder v4.6.0
    Checking dirs v1.0.5
    Checking errno v0.2.8
    Checking serde_core v1.0.228
    Checking getrandom v0.4.2
    Checking unicode-width v0.2.2
    Checking fallible-streaming-iterator v0.1.9
    Checking time-core v0.1.8
    Checking ucd-trie v0.1.7
    Checking linked-hash-map v0.5.6
    Checking fallible-iterator v0.3.0
    Checking regex-automata v0.4.14
    Checking unicode-segmentation v1.13.2
    Checking lazy_static v1.5.0
    Checking num-conv v0.2.1
    Checking pest v2.8.6
    Checking yaml-rust v0.4.5
    Checking regex v1.12.3
    Checking time v0.3.47
    Checking clap v4.6.1
    Checking lineread v0.7.4
    Checking structopt v0.3.26
    Checking errno v0.3.14
    Checking uuid v1.23.1
    Checking exec v0.3.1
    Checking nix v0.31.2
    Checking glob v0.3.3

warning: libsqlite3-sys@0.36.0: Compiler family detection failed due to error: ToolNotFound: failed to find tool "gcc.exe": program not found (see https://docs.rs/cc/latest/cc/#compile-time-requirements for help)
error: failed to run custom build command for `libsqlite3-sys v0.36.0`
note: To improve backtraces for build dependencies, set the CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true environment variable to enable debug information generation.
Caused by:
  process didn't exit successfully: `C:\Users\CKGuest\pls\target\debug\build\libsqlite3-sys-a6cb81ac3c4d76f0\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-env-changed=LIBSQLITE3_SYS_USE_PKG_CONFIG
  cargo:include=C:\Users\CKGuest\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\libsqlite3-sys-0.36.0/sqlite3
  cargo:rerun-if-changed=sqlite3/sqlite3.c
  cargo:rerun-if-changed=sqlite3/wasm32-wasi-vfs.c
  cargo:rerun-if-env-changed=SQLITE_MAX_VARIABLE_NUMBER
  cargo:rerun-if-env-changed=SQLITE_MAX_EXPR_DEPTH
  cargo:rerun-if-env-changed=SQLITE_MAX_COLUMN
  cargo:rerun-if-env-changed=LIBSQLITE3_FLAGS
  cargo:rerun-if-env-changed=CC_FORCE_DISABLE
  CC_FORCE_DISABLE = None
  cargo:rerun-if-env-changed=CC_x86_64-pc-windows-gnu
  CC_x86_64-pc-windows-gnu = None
  cargo:rerun-if-env-changed=CC_x86_64_pc_windows_gnu
  CC_x86_64_pc_windows_gnu = None
  cargo:rerun-if-env-changed=HOST_CC
  HOST_CC = None
  cargo:rerun-if-env-changed=CC
  CC = None
  cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
  cargo:warning=Compiler family detection failed due to error: ToolNotFound: failed to find tool "gcc.exe": program not found (see https://docs.rs/cc/latest/cc/#compile-time-requirements for help)
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = None
  cargo:rerun-if-env-changed=HOST_CFLAGS
  HOST_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS_x86_64_pc_windows_gnu
  CFLAGS_x86_64_pc_windows_gnu = None
  cargo:rerun-if-env-changed=CFLAGS_x86_64-pc-windows-gnu
  CFLAGS_x86_64-pc-windows-gnu = None
  --- stderr
    error occurred in cc-rs: failed to find tool "gcc.exe": program not found (see https://docs.rs/cc/latest/cc/#compile-time-requirements for help)
    warning: build failed, waiting for other jobs to finish...

error: failed to run custom build command for `libsqlite3-sys v0.36.0`
note: To improve backtraces for build dependencies, set the CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true environment variable to enable debug information generation.
Caused by:
  process didn't exit successfully: `C:\Users\CKGuest\pls\target\debug\build\libsqlite3-sys-a6cb81ac3c4d76f0\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-env-changed=LIBSQLITE3_SYS_USE_PKG_CONFIG
  cargo:include=C:\Users\CKGuest\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\libsqlite3-sys-0.36.0/sqlite3
  cargo:rerun-if-changed=sqlite3/sqlite3.c
  cargo:rerun-if-changed=sqlite3/wasm32-wasi-vfs.c
  cargo:rerun-if-env-changed=SQLITE_MAX_VARIABLE_NUMBER
  cargo:rerun-if-env-changed=SQLITE_MAX_EXPR_DEPTH
  cargo:rerun-if-env-changed=SQLITE_MAX_COLUMN
  cargo:rerun-if-env-changed=LIBSQLITE3_FLAGS
  cargo:rerun-if-env-changed=CC_FORCE_DISABLE
  CC_FORCE_DISABLE = None
  cargo:rerun-if-env-changed=CC_x86_64-pc-windows-gnu
  CC_x86_64-pc-windows-gnu = None
  cargo:rerun-if-env-changed=CC_x86_64_pc_windows_gnu
  CC_x86_64_pc_windows_gnu = None
  cargo:rerun-if-env-changed=HOST_CC
  HOST_CC = None
  cargo:rerun-if-env-changed=CC
  CC = None
  cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
  cargo:warning=Compiler family detection failed due to error: ToolNotFound: failed to find tool "gcc.exe": program not found (see https://docs.rs/cc/latest/cc/#compile-time-requirements for help)
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = None
  cargo:rerun-if-env-changed=HOST_CFLAGS
  HOST_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS_x86_64_pc_windows_gnu
  CFLAGS_x86_64_pc_windows_gnu = None
  cargo:rerun-if-env-changed=CFLAGS_x86_64-pc-windows-gnu
  CFLAGS_x86_64-pc-windows-gnu = None
  --- stderr
  error occurred in cc-rs: failed to find tool "gcc.exe": program not found (see https://docs.rs/cc/latest/cc/#compile-time-requirements for help)

warning: libsqlite3-sys@0.36.0: Compiler family detection failed due to error: ToolNotFound: failed to find tool "gcc.exe": program not found (see https://docs.rs/cc/latest/cc/#compile-time-requirements for help)
*/