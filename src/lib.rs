//! pls is a bash-like Unix shell written in Rust.
#![allow
(
    dead_code,
    unknown_lints,
    unused_imports,
)]

#[macro_use] pub mod macros
{

}

pub mod hash
{
    pub use std::hash::{ * };
}

pub mod lines
{
    use crate::
    {
        *,
    };

    #[derive(Debug)]
    pub struct LineInfo
    {
        // e.g. echo 'foo
        // is not a completed line, need to turn to multiple-line mode.
        pub tokens: tokens::Tokens,
        pub is_complete: bool,
    }

    impl LineInfo {
        pub fn new(tokens: tokens::Tokens) -> Self {
            LineInfo {
                tokens,
                is_complete: true,
            }
        }
    }
}

pub mod mem
{
    pub use std::mem::{ * };
}

pub mod reply
{
    use crate::{ * };

    #[derive(Clone, Debug, Default)]
    pub struct Reply {
        pub gid: i32,
        pub status: i32,
        pub stdout: String,
        pub stderr: String,
    }

    impl Reply
    {
        pub fn new() -> Reply {
            Reply {
                gid: 0,
                status: 0,
                stdout: String::new(),
                stderr: String::new(),
            }
        }

        pub fn from_status(gid: i32, status: i32) -> Reply {
            Reply {
                gid,
                status,
                stdout: String::new(),
                stderr: String::new(),
            }
        }

        pub fn error() -> Reply {
            Reply {
                gid: 0,
                status: 1,
                stdout: String::new(),
                stderr: String::new(),
            }
        }
    }
}

pub mod sync
{
    pub use std::sync::{ * };
}

pub mod tokens
{
    use crate::{ * };
    pub type Token = (String, String);
    pub type Tokens = Vec<Token>;
}

pub mod system
{
    use crate::
    {
        *,
    };
    //
    pub mod common
    {
        use crate::
        {
            *,
        };
        // Place libc here
    }
    //
    pub mod unistd
    {
        use crate::
        {
            *,
        };
        // Place libc here
    }
    // place nix here
}
/// Parse a command to tokens.
pub fn parse_line(cmd: &str) -> crate::lines::LineInfo
{
    crate::lines::LineInfo::new( cmd )
}
/// Run a command or a pipeline.
pub fn run(line: &str) -> crate::reply::Reply
{
    crate::reply::Reply::new()
}
