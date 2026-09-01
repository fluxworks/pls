// Copyright 2015, Yuheng Chen. See the LICENSE file at the top-level
// directory of this distribution.

//! YAML 1.2 implementation in pure Rust.
//!
//! # Usage
//!
//! This crate is [on github](https://github.com/chyh1990/yaml-rust) and can be
//! used by adding `yaml-rust` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies.yaml-rust]
//! git = "https://github.com/chyh1990/yaml-rust.git"
//! ```
//!
//! And this in your crate root:
//!
//! ```rust
//! extern crate yaml_rust;
//! ```
//!
//! Parse a string into `Vec<Yaml>` and then serialize it as a YAML string.
//!
//! # Examples
//!
//! ```
//! use yaml_rust::{YamlLoader, YamlEmitter};
//!
//! let docs = YamlLoader::load_from_str("[1, 2, 3]").unwrap();
//! let doc = &docs[0]; // select the first document
//! assert_eq!(doc[0].as_i64().unwrap(), 1); // access elements by index
//!
//! let mut out_str = String::new();
//! let mut emitter = YamlEmitter::new(&mut out_str);
//! emitter.dump(doc).unwrap(); // dump the YAML object to a String
//!
//! ```

#![doc(html_root_url = "https://docs.rs/yaml-rust/0.4.5")]

#![allow
(
    dead_code,
    mismatched_lifetime_syntaxes,
)]

extern crate linked_hash_map;

pub mod emitter;
pub mod parser;
pub mod scanner;
pub mod yaml;

// reexport key APIs
pub use crate::emitter::{EmitError, YamlEmitter};
pub use crate::parser::Event;
pub use crate::scanner::ScanError;
pub use crate::yaml::{Yaml, YamlLoader};

