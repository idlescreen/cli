// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Subcommand parsers for flag-only commands.

use super::{
    Cmd, CompletionShell, Lexer, ParseError, Tok, help_err, help_for, no_value, unknown_flag,
    unknown_short, usage_err,
};

pub(crate) fn parse_doctor(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<Cmd, ParseError> {
    let mut fix = false;
    let mut json = false;
    while let Some(t) = lx.next(&[])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('f') => fix = true,
            Tok::Short('j') => json = true,
            Tok::Short('h') => return Err(help_err(help_for(name))),
            Tok::Long(n, v) => match n.as_str() {
                "fix" => {
                    no_value(&n, v)?;
                    fix = true;
                }
                "json" => {
                    no_value(&n, v)?;
                    json = true;
                }
                _ => return Err(unknown_flag(&n)),
            },
            Tok::Pos(s) => {
                return Err(usage_err(format!("error: unexpected argument '{s}' found")));
            }
            Tok::Short(c) | Tok::ShortVal(c, _) => return Err(unknown_short(c)),
            Tok::End => {}
        }
    }
    Ok(Cmd::Doctor { fix, json })
}

pub(crate) fn parse_clean(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<Cmd, ParseError> {
    let mut dry_run = false;
    while let Some(t) = lx.next(&[])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('n') => dry_run = true,
            Tok::Short('h') => return Err(help_err(help_for(name))),
            Tok::Long(n, v) => match n.as_str() {
                "dry-run" => {
                    no_value(&n, v)?;
                    dry_run = true;
                }
                _ => return Err(unknown_flag(&n)),
            },
            Tok::Pos(s) => {
                return Err(usage_err(format!("error: unexpected argument '{s}' found")));
            }
            Tok::Short(c) | Tok::ShortVal(c, _) => return Err(unknown_short(c)),
            Tok::End => {}
        }
    }
    Ok(Cmd::Clean { dry_run })
}

pub(crate) fn parse_completion(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<Cmd, ParseError> {
    let mut shell = None;
    while let Some(t) = lx.next(&[])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('h') => return Err(help_err(help_for(name))),
            Tok::Short(c) | Tok::ShortVal(c, _) => return Err(unknown_short(c)),
            Tok::Long(n, v) => {
                no_value(&n, v)?;
                return Err(unknown_flag(&n));
            }
            Tok::Pos(s) => {
                if shell.is_some() {
                    return Err(usage_err(format!("error: unexpected argument '{s}' found")));
                }
                shell = Some(CompletionShell::parse(&s).ok_or_else(|| {
                            usage_err(format!(
                                "error: invalid value '{s}' for '<shell>' [possible values: bash, zsh, fish, nushell, powershell, elvish]"
                            ))
                        })?);
            }
            Tok::End => {}
        }
    }
    let Some(shell) = shell else {
        return Err(usage_err(
            "error: the following required arguments were not provided:\n  <shell>\n\nUsage: idlescreen completion <shell>\n\nFor more information, try '--help'.",
        ));
    };
    Ok(Cmd::Completion { shell })
}

pub(crate) fn parse_self_update(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<Cmd, ParseError> {
    let mut check = false;
    while let Some(t) = lx.next(&[])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('c') => check = true,
            Tok::Short('h') => return Err(help_err(help_for(name))),
            Tok::Long(n, v) => match n.as_str() {
                "check" => {
                    no_value(&n, v)?;
                    check = true;
                }
                _ => return Err(unknown_flag(&n)),
            },
            Tok::Pos(s) => {
                return Err(usage_err(format!("error: unexpected argument '{s}' found")));
            }
            Tok::Short(c) | Tok::ShortVal(c, _) => return Err(unknown_short(c)),
            Tok::End => {}
        }
    }
    Ok(Cmd::SelfUpdate { check })
}

pub(crate) fn parse_version(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<Cmd, ParseError> {
    let mut long = false;
    let mut json = false;
    while let Some(t) = lx.next(&[])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('l') => long = true,
            Tok::Short('j') => json = true,
            Tok::Short('h') => return Err(help_err(help_for(name))),
            Tok::Long(n, v) => match n.as_str() {
                "long" => {
                    no_value(&n, v)?;
                    long = true;
                }
                "json" => {
                    no_value(&n, v)?;
                    json = true;
                }
                _ => return Err(unknown_flag(&n)),
            },
            Tok::Pos(s) => {
                return Err(usage_err(format!("error: unexpected argument '{s}' found")));
            }
            Tok::Short(c) | Tok::ShortVal(c, _) => return Err(unknown_short(c)),
            Tok::End => {}
        }
    }
    Ok(Cmd::Version { long, json })
}
