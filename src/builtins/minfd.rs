use std::io::Write;

use crate::builtins::utils::print_stdout_with_capture;
use crate::shell::Shell;
use crate::types::{Command, CommandLine, CommandResult};

pub fn run(_sh: &mut Shell, cl: &CommandLine, cmd: &Command, capture: bool) -> CommandResult
{
    let mut cr = CommandResult::new();
    println_stderr!(":: minfd: error: Not Supported On The Windows Platform");
    cr
}
