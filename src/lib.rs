//! > DESCRIPTION

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod checker;
mod wordlist_codegen;

pub use checker::*;
