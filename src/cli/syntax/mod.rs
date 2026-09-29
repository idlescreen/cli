//! Syntax definitions: error types, flags parsing, and lexer.

pub mod error;
pub mod flags;
pub mod lexer;

pub use error::*;
pub(crate) use flags::*;
pub(crate) use lexer::*;
