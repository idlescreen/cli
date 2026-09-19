// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Hand-rolled argument parser for `idlescreen` (replaces clap). All
//! legacy spellings survive as aliases so they show up in `--help`.

mod cmd;
mod cmd_args;
mod cmd_flags;
mod error;
mod flags;
mod help;
mod inhibit;
mod lexer;
mod nested;

pub use cmd::{Cli, Cmd, CompletionShell, ConfigOp, OverlayState, SaverOp};
pub use error::{ErrorKind, ParseError};

pub(crate) use cmd_args::*;
pub(crate) use cmd_flags::*;
pub(crate) use error::{
    help_err, no_value, parse_u32, parse_u64, unknown_flag, unknown_short, usage_err,
};
pub(crate) use flags::*;
pub(crate) use help::{help_for, help_top};
pub(crate) use inhibit::*;
pub(crate) use lexer::{Lexer, Tok};
pub(crate) use nested::*;

impl Cli {
    /// clap-compatible entry point: first item is the program name.
    pub fn try_parse_from<I, S>(itr: I) -> Result<Self, ParseError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let argv: Vec<String> = itr.into_iter().map(|s| s.as_ref().to_string()).collect();
        let args = if argv.is_empty() { &[][..] } else { &argv[1..] };
        Self::parse(args)
    }

    fn parse(args: &[String]) -> Result<Self, ParseError> {
        let mut quiet = false;
        let mut i = 0;
        // Global flags may precede the subcommand: -q/--quiet, -h/--help,
        // -V/--version. Short clusters are split (-qq → q,q).
        while i < args.len() {
            let a = &args[i];
            if a == "--" {
                return Err(usage_err(
                    "error: unexpected argument '--' found; expected a subcommand",
                ));
            }
            if let Some(l) = a.strip_prefix("--") {
                let (name, val) = match l.split_once('=') {
                    Some((n, v)) => (n, Some(v)),
                    None => (l, None),
                };
                match name {
                    "quiet" => {
                        no_value(name, val.map(String::from))?;
                        quiet = true;
                    }
                    "help" => return Err(help_err(help_top())),
                    "version" => {
                        return Err(ParseError::new(ErrorKind::DisplayVersion, version_text()));
                    }
                    other => return Err(unknown_flag(other)),
                }
                i += 1;
                continue;
            }
            if let Some(s) = a.strip_prefix('-') {
                if s.is_empty() {
                    return Err(usage_err("error: unexpected argument '-' found"));
                }
                if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                    return Err(usage_err(format!(
                        "error: unexpected argument '{a}' found; expected a subcommand"
                    )));
                }
                for c in s.chars() {
                    match c {
                        'q' => quiet = true,
                        'h' => return Err(help_err(help_top())),
                        'V' => {
                            return Err(ParseError::new(ErrorKind::DisplayVersion, version_text()));
                        }
                        other => return Err(unknown_short(other)),
                    }
                }
                i += 1;
                continue;
            }
            break;
        }

        let Some(word) = args.get(i) else {
            return Err(ParseError::new(
                ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
                help_top(),
            ));
        };
        let rest = &args[i + 1..];
        let cmd = dispatch(word, rest, &mut quiet)?;
        Ok(Self { quiet, cmd })
    }
}

fn version_text() -> String {
    format!("idlescreen {}", env!("CARGO_PKG_VERSION"))
}
fn canonical(word: &str) -> Option<&'static str> {
    Some(match word {
        "status" | "st" => "status",
        "config" | "cfg" => "config",
        "enable" | "on" => "enable",
        "disable" | "off" => "disable",
        "timeout" | "t" => "timeout",
        "saver" | "sv" => "saver",
        "list" | "ls" => "list",
        "inhibitors" | "inhib" => "inhibitors",
        "inhibit" | "hold" => "inhibit",
        "start" | "activate" => "start",
        "preview" | "p" => "preview",
        "stop" | "x" => "stop",
        "fps-overlay" | "fps" => "fps-overlay",
        "render-scale" | "scale" => "render-scale",
        "interactive" | "i" => "interactive",
        "restart" | "rs" => "restart",
        "logs" | "log" => "logs",
        "doctor" | "doc" => "doctor",
        "clean" | "cl" => "clean",
        "completion" | "comp" => "completion",
        "bug-report" | "bug" => "bug-report",
        "self-update" | "update" | "upgrade" => "self-update",
        "tui" | "ui" => "tui",
        "version" | "v" => "version",
        "about" | "info" => "about",
        "help" => "help",
        _ => return None,
    })
}

fn dispatch(word: &str, args: &[String], quiet: &mut bool) -> Result<Cmd, ParseError> {
    let Some(name) = canonical(word) else {
        return Err(usage_err(format!(
            "error: unrecognized subcommand '{word}'\n\nUsage: idlescreen <COMMAND>\n\nFor more information, try '--help'."
        )));
    };
    if name == "help" {
        return match args.first() {
            None => Err(help_err(help_top())),
            Some(topic) => match canonical(topic) {
                Some(t) if t != "help" => Err(help_err(help_for(t))),
                _ => Err(usage_err(format!(
                    "error: unrecognized subcommand '{topic}'"
                ))),
            },
        };
    }
    let mut lx = Lexer::new(args);
    match name {
        "status" => Ok(Cmd::Status {
            json: flags_json(&mut lx, quiet, name)?,
        }),
        "enable" => {
            flags_none(&mut lx, quiet, name)?;
            Ok(Cmd::Enable)
        }
        "disable" => {
            flags_none(&mut lx, quiet, name)?;
            Ok(Cmd::Disable)
        }
        "start" => {
            flags_none(&mut lx, quiet, name)?;
            Ok(Cmd::Start)
        }
        "stop" => {
            flags_none(&mut lx, quiet, name)?;
            Ok(Cmd::Stop)
        }
        "interactive" => {
            flags_none(&mut lx, quiet, name)?;
            Ok(Cmd::Interactive)
        }
        "restart" => {
            flags_none(&mut lx, quiet, name)?;
            Ok(Cmd::Restart)
        }
        "bug-report" => {
            flags_none(&mut lx, quiet, name)?;
            Ok(Cmd::BugReport)
        }
        "about" => {
            flags_none(&mut lx, quiet, name)?;
            Ok(Cmd::About)
        }
        "list" => Ok(Cmd::List {
            json: flags_json(&mut lx, quiet, name)?,
        }),
        "inhibitors" => Ok(Cmd::Inhibitors {
            json: flags_json(&mut lx, quiet, name)?,
        }),
        "timeout" => parse_timeout(&mut lx, quiet, name),
        "preview" => parse_preview(&mut lx, quiet, name),
        "config" => parse_config(&mut lx, quiet),
        "saver" => parse_saver(&mut lx, quiet),
        "inhibit" => parse_inhibit(args, quiet),
        "fps-overlay" => parse_fps_overlay(&mut lx, quiet, name),
        "render-scale" => parse_render_scale(&mut lx, quiet, name),
        "logs" => parse_logs(&mut lx, quiet, name),
        "doctor" => parse_doctor(&mut lx, quiet, name),
        "clean" => parse_clean(&mut lx, quiet, name),
        "completion" => parse_completion(&mut lx, quiet, name),
        "self-update" => parse_self_update(&mut lx, quiet, name),
        "tui" => Ok(Cmd::Tui {
            args: args.to_vec(),
        }),
        "version" => parse_version(&mut lx, quiet, name),
        _ => Err(usage_err(format!("error: unrecognized subcommand '{name}'"))),
    }
}
