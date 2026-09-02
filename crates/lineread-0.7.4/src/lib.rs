//! Provides a configurable, concurrent, extensible, interactive input reader for Unix terminals and Windows console.
#![deny(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

pub use crate::command::Command;
pub use crate::complete::{Completer, Completion, Suffix};
pub use crate::function::Function;
pub use crate::interface::Interface;
pub use crate::prompter::Prompter;
pub use crate::reader::{ReadResult, Reader};
pub use crate::terminal::{DefaultTerminal, Signal, Terminal};
pub use crate::writer::Writer;

pub mod chars;
pub mod command;
pub mod complete;
pub mod function;
pub mod highlighting;
pub mod inputrc;
pub mod interface;
pub mod memory;
pub mod prompter;
pub mod reader;
pub mod table;
pub mod terminal;
pub mod util;
pub mod variables;
pub mod writer;

#[cfg(unix)]
#[path = "unix/mod.rs"]
mod sys;

#[cfg(windows)]
#[path = "windows/mod.rs"]
mod sys;
