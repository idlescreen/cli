// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Shared flag helpers used by the subcommand parsers.

use super::{
    Lexer, ParseError, Tok, help_err, help_for, no_value, unknown_flag, unknown_short, usage_err,
};

/// A `--flag value` or `--flag=value` pair.
pub(crate) fn flag_value(
    lx: &mut Lexer<'_>,
    name: &str,
    inline: Option<String>,
) -> Result<String, ParseError> {
    if let Some(v) = inline {
        return Ok(v);
    }
    match lx.next(&[])? {
        Some(Tok::Pos(s)) => Ok(s),
        _ => Err(usage_err(format!(
            "error: a value is required for '--{name}' but none was supplied"
        ))),
    }
}

/// Commands accepting only `-q/--quiet` and `-h/--help`.
pub(crate) fn flags_none(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<(), ParseError> {
    while let Some(t) = lx.next(&[])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('h') => return Err(help_err(help_for(name))),
            Tok::Long(n, v) if n == "help" => {
                no_value(&n, v)?;
                return Err(help_err(help_for(name)));
            }
            Tok::Long(n, v) => {
                no_value(&n, v)?;
                return Err(unknown_flag(&n));
            }
            Tok::Pos(s) => {
                return Err(usage_err(format!("error: unexpected argument '{s}' found")));
            }
            Tok::Short(c) | Tok::ShortVal(c, _) => return Err(unknown_short(c)),
            Tok::End => {}
        }
    }
    Ok(())
}

/// Commands accepting `-q/--quiet`, `-j/--json`, `-h/--help`.
pub(crate) fn flags_json(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<bool, ParseError> {
    let mut json = false;
    while let Some(t) = lx.next(&[])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('j') => json = true,
            Tok::Short('h') => return Err(help_err(help_for(name))),
            Tok::Long(n, v) if n == "json" => {
                no_value(&n, v)?;
                json = true;
            }
            Tok::Long(n, v) if n == "help" => {
                no_value(&n, v)?;
                return Err(help_err(help_for(name)));
            }
            Tok::Long(n, v) => {
                no_value(&n, v)?;
                return Err(unknown_flag(&n));
            }
            Tok::Pos(s) => {
                return Err(usage_err(format!("error: unexpected argument '{s}' found")));
            }
            Tok::Short(c) | Tok::ShortVal(c, _) => return Err(unknown_short(c)),
            Tok::End => {}
        }
    }
    Ok(json)
}
