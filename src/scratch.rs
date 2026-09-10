//! pls is a bash-like Unix shell written in Rust.
#![allow(dead_code)]
#![allow(unknown_lints)]
// #![feature(tool_lints)]
extern crate errno;
extern crate exec;
extern crate glob;
//extern crate libc;
extern crate lineread;
//extern crate nix;
extern crate regex;
extern crate rusqlite;

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

#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ctime;
mod types;

#[macro_use]
mod tlog;
#[macro_use]
mod tools;

mod builtins;
mod calculator;
mod core;
mod execute;
mod history;
mod jobc;
mod libs;
mod parsers;
mod rcfile;
mod scripting;
mod shell;
mod signals;

/// Represents an error calling `exec`.
pub use crate::types::CommandResult;
pub use crate::types::LineInfo;

/// Parse a command to tokens.
pub fn parse_line(cmd: &str) -> LineInfo {
    parsers::parser_line::parse_line(cmd)
}

/// Run a command or a pipeline.
pub fn run(line: &str) -> CommandResult {
    execute::run(line)
}
