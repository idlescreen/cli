// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! `inhibit` — has its own arg loop (not Lexer-driven).

use super::*;

/// `inhibit [-r reason] <command...>` — flags parsed until the first
/// positional; everything after (hyphens included) is the child argv,
/// captured verbatim.
pub(crate) fn parse_inhibit(args: &[String], quiet: &mut bool) -> Result<Cmd, ParseError> {
    let mut reason = None;
    let mut command: Vec<String> = Vec::new();
    let mut i = 0;
    'outer: while i < args.len() {
        let a = &args[i];
        i += 1;
        if a == "--" {
            command.extend_from_slice(&args[i..]);
            break;
        }
        if let Some(l) = a.strip_prefix("--") {
            let (n, v) = match l.split_once('=') {
                Some((n, v)) => (n, Some(v.to_string())),
                None => (l, None),
            };
            match n {
                "quiet" => *quiet = true,
                "help" => return Err(help_err(help_for("inhibit"))),
                "reason" => {
                    reason = Some(match v {
                        Some(v) => v,
                        None => match args.get(i) {
                            Some(s) => {
                                i += 1;
                                s.clone()
                            }
                            None => {
                                return Err(usage_err(
                                    "error: a value is required for '--reason' but none was supplied",
                                ));
                            }
                        },
                    });
                }
                _ => {
                    // Unknown long flag starts the captured child argv.
                    command.push(a.clone());
                    command.extend_from_slice(&args[i..]);
                    break;
                }
            }
            continue;
        }
        if let Some(s) = a.strip_prefix('-') {
            if !s.is_empty() && !s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                // Short cluster: bools emit; 'r' takes the rest/next as value;
                // anything unknown starts verbatim capture.
                let chars: Vec<char> = s.chars().collect();
                let mut k = 0;
                while k < chars.len() {
                    match chars[k] {
                        'q' => *quiet = true,
                        'h' => return Err(help_err(help_for("inhibit"))),
                        'r' => {
                            let rest: String = chars[k + 1..].iter().collect();
                            if rest.is_empty() {
                                match args.get(i) {
                                    Some(s) => {
                                        i += 1;
                                        reason = Some(s.clone());
                                    }
                                    None => {
                                        return Err(usage_err(
                                            "error: a value is required for '-r' but none was supplied",
                                        ));
                                    }
                                }
                            } else {
                                reason = Some(rest.strip_prefix('=').unwrap_or(&rest).to_string());
                            }
                            break;
                        }
                        _ => {
                            command.push(a.clone());
                            command.extend_from_slice(&args[i..]);
                            break 'outer;
                        }
                    }
                    k += 1;
                }
                continue;
            }
        }
        // Positional (or "-", or a negative number): capture verbatim.
        command.push(a.clone());
        command.extend_from_slice(&args[i..]);
        break;
    }
    if command.is_empty() {
        return Err(usage_err(
            "error: the following required arguments were not provided:\n  <command>\n\nUsage: idlescreen inhibit <command>\n\nFor more information, try '--help'.",
        ));
    }
    Ok(Cmd::Inhibit { reason, command })
}
