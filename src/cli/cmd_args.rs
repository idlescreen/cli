// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Subcommand parsers that take positional values or mixed flags.

use super::{
    Cmd, Lexer, OverlayState, ParseError, Tok, flag_value, help_err, help_for, no_value, parse_u32,
    parse_u64, unknown_flag, unknown_short, usage_err,
};

pub(crate) fn parse_timeout(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<Cmd, ParseError> {
    let mut minutes = None;
    let mut json = false;
    while let Some(t) = lx.next(&[])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('j') => json = true,
            Tok::Short('h') => return Err(help_err(help_for(name))),
            Tok::Short(c) | Tok::ShortVal(c, _) => return Err(unknown_short(c)),
            Tok::Long(n, v) => match n.as_str() {
                "json" => {
                    no_value(&n, v)?;
                    json = true;
                }
                "help" => {
                    no_value(&n, v)?;
                    return Err(help_err(help_for(name)));
                }
                _ => return Err(unknown_flag(&n)),
            },
            Tok::Pos(s) => {
                if minutes.is_some() {
                    return Err(usage_err(format!("error: unexpected argument '{s}' found")));
                }
                minutes = Some(parse_u32(&s, "'<minutes>'")?);
            }
            Tok::End => {}
        }
    }
    Ok(Cmd::Timeout { minutes, json })
}

pub(crate) fn parse_preview(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<Cmd, ParseError> {
    let mut pname: Option<String> = None;
    let mut timeout = None;
    while let Some(t) = lx.next(&['t'])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('h') => return Err(help_err(help_for(name))),
            Tok::ShortVal('t', v) => timeout = Some(parse_u64(&v, "'--timeout'")?),
            Tok::Short(c) | Tok::ShortVal(c, _) => return Err(unknown_short(c)),
            Tok::Long(n, v) => match n.as_str() {
                "timeout" => {
                    let v = flag_value(lx, &n, v)?;
                    timeout = Some(parse_u64(&v, "'--timeout'")?);
                }
                "help" => {
                    no_value(&n, v)?;
                    return Err(help_err(help_for(name)));
                }
                _ => return Err(unknown_flag(&n)),
            },
            Tok::Pos(s) => {
                if pname.is_some() {
                    return Err(usage_err(format!("error: unexpected argument '{s}' found")));
                }
                pname = Some(s);
            }
            Tok::End => {}
        }
    }
    let Some(name) = pname else {
        return Err(usage_err(
            "error: the following required arguments were not provided:\n  <name>\n\nUsage: idlescreen preview <name>\n\nFor more information, try '--help'.",
        ));
    };
    Ok(Cmd::Preview { name, timeout })
}

pub(crate) fn parse_fps_overlay(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<Cmd, ParseError> {
    let mut state = None;
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
                if state.is_some() {
                    return Err(usage_err(format!("error: unexpected argument '{s}' found")));
                }
                state = Some(match s.as_str() {
                    "on" => OverlayState::On,
                    "off" => OverlayState::Off,
                    "status" => OverlayState::Status,
                    _ => {
                        return Err(usage_err(format!(
                            "error: invalid value '{s}' for '<state>' [possible values: on, off, status]"
                        )));
                    }
                });
            }
            Tok::End => {}
        }
    }
    Ok(Cmd::FpsOverlay { state })
}

pub(crate) fn parse_render_scale(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<Cmd, ParseError> {
    let mut value = None;
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
                if value.is_some() {
                    return Err(usage_err(format!("error: unexpected argument '{s}' found")));
                }
                value = Some(s);
            }
            Tok::End => {}
        }
    }
    Ok(Cmd::RenderScale { value })
}

pub(crate) fn parse_logs(
    lx: &mut Lexer<'_>,
    quiet: &mut bool,
    name: &'static str,
) -> Result<Cmd, ParseError> {
    let mut follow = false;
    let mut lines = 100u32;
    while let Some(t) = lx.next(&['n'])? {
        match t {
            Tok::Short('q') => *quiet = true,
            Tok::Short('f') => follow = true,
            Tok::Short('h') => return Err(help_err(help_for(name))),
            Tok::ShortVal('n', v) => lines = parse_u32(&v, "'--lines'")?,
            Tok::Long(n, v) => match n.as_str() {
                "follow" => {
                    no_value(&n, v)?;
                    follow = true;
                }
                "lines" => {
                    let v = flag_value(lx, &n, v)?;
                    lines = parse_u32(&v, "'--lines'")?;
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
    Ok(Cmd::Logs { follow, lines })
}
