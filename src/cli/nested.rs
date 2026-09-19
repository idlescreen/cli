// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Nested subcommand parsers: config and saver.

use super::*;

/// `config [op]` — `--json` is global across the config subtree.
pub(crate) fn parse_config(lx: &mut Lexer<'_>, quiet: &mut bool) -> Result<Cmd, ParseError> {
    let mut json = false;
    // First token decides the sub-operation (or a flag).
    let mut op: Option<ConfigOp> = None;
    while let Some(t) = lx.next(&[])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('j') => json = true,
            Tok::Short('h') => return Err(help_err(help_for("config"))),
            Tok::Long(n, v) if n == "json" => {
                no_value(&n, v)?;
                json = true;
            }
            Tok::Long(n, v) if n == "help" => {
                no_value(&n, v)?;
                return Err(help_err(help_for("config")));
            }
            Tok::Pos(word) => {
                if op.is_some() {
                    return Err(usage_err(format!(
                        "error: unexpected argument '{word}' found"
                    )));
                }
                op = Some(parse_config_op(lx, &word, quiet, &mut json)?);
            }
            Tok::Long(n, v) => {
                no_value(&n, v)?;
                return Err(unknown_flag(&n));
            }
            Tok::Short(c) | Tok::ShortVal(c, _) => return Err(unknown_short(c)),
            Tok::End => {}
        }
    }
    Ok(Cmd::Config { op, json })
}

pub(crate) fn parse_config_op(
    lx: &mut Lexer<'_>,
    word: &str,
    quiet: &mut bool,
    json: &mut bool,
) -> Result<ConfigOp, ParseError> {
    match word {
        "list" => {
            finish_nested(lx, quiet, json, "config list")?;
            Ok(ConfigOp::List)
        }
        "path" => {
            finish_nested(lx, quiet, json, "config path")?;
            Ok(ConfigOp::Path)
        }
        "edit" => {
            finish_nested(lx, quiet, json, "config edit")?;
            Ok(ConfigOp::Edit)
        }
        "get" => {
            let key = required_pos(lx, "<key>", "config get")?;
            finish_nested(lx, quiet, json, "config get")?;
            Ok(ConfigOp::Get { key })
        }
        "set" => {
            let key = required_pos(lx, "<key>", "config set")?;
            let value = required_pos(lx, "<value>", "config set")?;
            finish_nested(lx, quiet, json, "config set")?;
            Ok(ConfigOp::Set { key, value })
        }
        "reset" => {
            let mut yes = false;
            while let Some(t) = lx.next(&[])? {
                match t {
                    Tok::Short('q') => *quiet = true,
                    Tok::Short('y') => yes = true,
                    Tok::Short('j') => *json = true,
                    Tok::Short('h') => return Err(help_err(help_for("config"))),
                    Tok::Long(n, v) => match n.as_str() {
                        "yes" => {
                            no_value(&n, v)?;
                            yes = true;
                        }
                        "json" => {
                            no_value(&n, v)?;
                            *json = true;
                        }
                        "help" => {
                            no_value(&n, v)?;
                            return Err(help_err(help_for("config")));
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
            Ok(ConfigOp::Reset { yes })
        }
        other => Err(usage_err(format!(
            "error: unrecognized subcommand '{other}'\n\nUsage: idlescreen config <COMMAND>\n\nFor more information, try '--help'."
        ))),
    }
}

/// `saver [op]` — `--json` global across the subtree.
pub(crate) fn parse_saver(lx: &mut Lexer<'_>, quiet: &mut bool) -> Result<Cmd, ParseError> {
    let mut json = false;
    let mut op: Option<SaverOp> = None;
    while let Some(t) = lx.next(&[])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('j') => json = true,
            Tok::Short('h') => return Err(help_err(help_for("saver"))),
            Tok::Long(n, v) if n == "json" => {
                no_value(&n, v)?;
                json = true;
            }
            Tok::Long(n, v) if n == "help" => {
                no_value(&n, v)?;
                return Err(help_err(help_for("saver")));
            }
            Tok::Pos(word) => {
                if op.is_some() {
                    return Err(usage_err(format!(
                        "error: unexpected argument '{word}' found"
                    )));
                }
                op = Some(match word.as_str() {
                    "list" => {
                        finish_nested(lx, quiet, &mut json, "saver list")?;
                        SaverOp::List
                    }
                    "set" => {
                        let name = required_pos(lx, "<name>", "saver set")?;
                        finish_nested(lx, quiet, &mut json, "saver set")?;
                        SaverOp::Set { name }
                    }
                    other => {
                        return Err(usage_err(format!(
                            "error: unrecognized subcommand '{other}'\n\nUsage: idlescreen saver <COMMAND>\n\nFor more information, try '--help'."
                        )));
                    }
                });
            }
            Tok::Long(n, v) => {
                no_value(&n, v)?;
                return Err(unknown_flag(&n));
            }
            Tok::Short(c) | Tok::ShortVal(c, _) => return Err(unknown_short(c)),
            Tok::End => {}
        }
    }
    Ok(Cmd::Saver { op, json })
}

/// Remaining args of a nested op: only global flags may follow.
pub(crate) fn finish_nested(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    json: &mut bool,
    _ctx: &str,
) -> Result<(), ParseError> {
    while let Some(t) = lx.next(&[])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('j') => *json = true,
            Tok::Long(n, v) if n == "json" => {
                no_value(&n, v)?;
                *json = true;
            }
            Tok::Long(n, v) if n == "quiet" => {
                no_value(&n, v)?;
                *quiet = true;
            }
            Tok::Long(n, v) if n == "help" => {
                no_value(&n, v)?;
                return Err(help_err(help_top()));
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

pub(crate) fn required_pos(
    lx: &mut Lexer<'_>,
    what: &str,
    ctx: &str,
) -> Result<String, ParseError> {
    match lx.next(&[])? {
        Some(Tok::Pos(s)) => Ok(s),
        _ => Err(usage_err(format!(
            "error: the following required arguments were not provided:\n  {what}\n\nUsage: idlescreen {ctx} {what}\n\nFor more information, try '--help'."
        ))),
    }
}
