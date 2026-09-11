//! pls is a bash-like Unix shell written in Rust.
#![allow
(
    dead_code,
    unknown_lints,
    unused_imports,
    unused_unsafe,
    unused_variables,
)]

extern crate regex as re;

#[macro_use] pub mod macros
{

}

pub mod api
{
    use crate::
    {
        *,
    };

    pub const API:[&str; 20] =  [ "alias", "bg", "cd", "check", "cinfo", "exec", "exit", "export", "fg", "history", "jobs", "read", "source", "ulimit", "unalias", "vox", "minfd", "set", "unset", "unpath", ];
}

pub mod collections
{
    pub use std::collections::{ * };
}

pub mod env
{
    pub use std::env::{ * };
    use crate::
    {
        *,
    };
}

pub mod hash
{
    pub use std::hash::{ * };
}

pub mod is
{
    use crate::
    {
        *,
    };

    //pub fn is_builtin(s: &str) -> bool
    pub fn api(s: &str) -> bool
    {
        unsafe
            {
                api::API.contains( &s )
            }
    }

    //pub fn is_arithmetic(line: &str) -> bool
    pub fn arithmetic(line: &str) -> bool
    {
        if !regex::contains(line, r"[0-9]+") { return false; }

        if !regex::contains(line, r"\+|\-|\*|/|\^") { return false; }

        regex::contains(line, r"^[ 0-9\.\(\)\+\-\*/\^]+[\.0-9 \)]$")
    }

    //pub fn is_env(line: &str) -> bool
    pub fn environment(line: &str) -> bool { regex::contains(line, r"^[a-zA-Z_][a-zA-Z0-9_]*=.*$") }

    //pub fn is_shell_altering_command(line: &str) -> bool {
    pub fn shell_altering_command(line: &str) -> bool
    {
        let line = line.trim();

        if regex::contains(line, r"^[A-Za-z_][A-Za-z0-9_]*=.*$") { return true; }

        line.starts_with("alias ")
        || line.starts_with("export ")
        || line.starts_with("unalias ")
        || line.starts_with("unset ")
        || line.starts_with("source ")
    }
    //pub fn is_signal_handler_enabled() -> bool
    pub fn signal_handler_enabled() -> bool { env::var("PLS_ENABLE_SIG_HANDLER").is_ok_and(|x| x == "1") }
}

pub mod lines
{
    use crate::
    {
        tokens::{ Tokens },
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

    impl LineInfo
    {
        pub const fn new() -> Self
        {
            Self
            {
                tokens: Tokens::new(),
                is_complete: false,
            }
        }
        pub fn create(tokens: tokens::Tokens) -> Self {
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

pub mod now
{
    use crate::
    {
        *,
    };
}

pub mod path
{
    pub use std::path::{ * };
    use crate::
    {
        collections::{ HashSet },
        *,
    };

    pub const DRIVE:&str = r#"C:"#;
    pub const OPERATING_SYSTEM:&str = r#"\\WINDOWS"#;
    pub const SYSTEM:&str = r#"\\SYSTEM32"#;

    pub fn initialize_environment()
    {
        unsafe
        {
            let mut all_paths: HashSet<PathBuf> = HashSet::new();
            for x in
                [
                    format!( r#"{}{}"#, DRIVE, OPERATING_SYSTEM ),
                    format!( r#"{}{}{}"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\Drivers"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\Drivers\\DriverData"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\OpenSSH"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\Wbem"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\WindowsPowerShell"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
                    format!( r#"{}{}{}\\WindowsPowerShell\\v1.0"#, DRIVE, OPERATING_SYSTEM, SYSTEM ),
            ]
            {
            let path_buf = PathBuf::from(x);
            if path_buf.exists() {
            all_paths.insert(path_buf);
            }
            }

            if let Ok(env_path) = env::var("PATH") {
                for one_path in env::split_paths(&env_path) {
                    if !all_paths.contains(&one_path) {
                        all_paths.insert(one_path);
                    }
                }
            }
            let path_var = env::join_paths(all_paths).unwrap_or_default();
            env::set_var("PATH", path_var);
        }
    }

    pub fn is_shell_altering_command(line: &str) -> bool {
        let line = line.trim();
        if re_contains(line, r"^[A-Za-z_][A-Za-z0-9_]*=.*$") {
            return true;
        }
        line.starts_with("alias ")
            || line.starts_with("export ")
            || line.starts_with("unalias ")
            || line.starts_with("unset ")
            || line.starts_with("source ")
    }
}

pub mod parses
{
    use crate::
    {
        *,
    };

    pub mod line
    {
        use crate::
        {
            lines::{ LineInfo },
            *,
        };

        pub fn parses(line: &str) -> LineInfo
        {
            let parsed = LineInfo::new();
            let mut result = Vec::new();

            if is::arithmetic( line )
            {
                for x in line.split(' ')
                {
                    result.push((String::from(""), x.to_string()));
                }

                return LineInfo::create( result );
            }

            parsed
        }
    }
}

pub mod regex
{
    pub use crate::re::{ * };
    use crate::
    {
        *,
    };

    pub fn contains(text: &str, ptn: &str) -> bool
    {
        let re = match Regex::new(ptn)
        {
            Ok(x) => x,
            Err(e) => {
                println!("Regex new error: {:?}", e);
                return false;
            }
        };

        re.is_match(text)
    }
}

pub mod replies
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
    crate::parses::line::parses( cmd )
    //crate::lines::LineInfo::new( cmd )
}
/// Run a command or a pipeline.
pub fn run( line:&str ) -> crate::replies::Reply
{
    crate::replies::Reply::new()
}
