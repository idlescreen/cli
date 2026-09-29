// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! ParseError/ErrorKind + shared error/int-parsing helpers.

use std::fmt;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    DisplayHelp,
    DisplayHelpOnMissingArgumentOrSubcommand,
    DisplayVersion,
    Usage,
}

/// Parse failure or display request. `print()` renders like clap did:
/// help/version to stdout, usage errors to stderr.
#[derive(Debug)]
pub struct ParseError {
    kind: ErrorKind,
    text: String,
}

impl ParseError {
    pub(crate) fn new(kind: ErrorKind, text: impl Into<String>) -> Self {
        Self {
            kind,
            text: text.into(),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn print(&self) -> std::io::Result<()> {
        match self.kind {
            ErrorKind::DisplayHelp
            | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            | ErrorKind::DisplayVersion => {
                writeln!(std::io::stdout().lock(), "{}", self.text)
            }
            ErrorKind::Usage => {
                writeln!(std::io::stderr().lock(), "{}", self.text)
            }
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}
impl std::error::Error for ParseError {}

pub(crate) fn usage_err(msg: impl Into<String>) -> ParseError {
    ParseError::new(ErrorKind::Usage, msg)
}

pub(crate) fn help_err(text: impl Into<String>) -> ParseError {
    ParseError::new(ErrorKind::DisplayHelp, text)
}
pub(crate) fn no_value(name: &str, val: Option<String>) -> Result<(), ParseError> {
    if val.is_some() {
        return Err(usage_err(format!(
            "error: unexpected value for '--{name}' found; no more were expected"
        )));
    }
    Ok(())
}

pub(crate) fn unknown_flag(name: &str) -> ParseError {
    usage_err(format!("error: unexpected argument '--{name}' found"))
}

pub(crate) fn unknown_short(c: char) -> ParseError {
    usage_err(format!("error: unexpected argument '-{c}' found"))
}

pub(crate) fn parse_u32(v: &str, what: &str) -> Result<u32, ParseError> {
    v.parse().map_err(|_| {
        usage_err(format!(
            "error: invalid value '{v}' for {what}: expected a number"
        ))
    })
}

pub(crate) fn parse_u64(v: &str, what: &str) -> Result<u64, ParseError> {
    v.parse().map_err(|_| {
        usage_err(format!(
            "error: invalid value '{v}' for {what}: expected a number"
        ))
    })
}
