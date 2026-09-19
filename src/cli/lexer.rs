// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

use super::*;

// Lexer: clap-compatible short-cluster / long / `--` / passthrough handling.
pub(crate) enum Tok {
    /// Positional argument.
    Pos(String),
    /// Boolean short flag, e.g. `-j` (or one member of `-jq`).
    Short(char),
    /// Value-taking short flag: `-t 5`, `-t5`, `-t=5`.
    ShortVal(char, String),
    /// `--long` or `--long=value`.
    Long(String, Option<String>),
    /// `--` end-of-flags marker.
    End,
}

pub(crate) struct Lexer<'a> {
    args: &'a [String],
    i: usize,
    /// Remaining chars of a short cluster being split (`-jq` → `j`,`q`).
    pending: Vec<char>,
    end_of_flags: bool,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(args: &'a [String]) -> Self {
        Self {
            args,
            i: 0,
            pending: Vec::new(),
            end_of_flags: false,
        }
    }

    /// `val_shorts`: short flags that consume a value (`-t 5` style).
    pub(crate) fn next(&mut self, val_shorts: &[char]) -> Result<Option<Tok>, ParseError> {
        if !self.pending.is_empty() {
            let c = self.pending.remove(0);
            if val_shorts.contains(&c) {
                let v = if !self.pending.is_empty() {
                    // `-t5` or `-t=5`: the rest of the cluster is the value.
                    let mut s: String = self.pending.drain(..).collect();
                    if let Some(r) = s.strip_prefix('=') {
                        s = r.to_string();
                    }
                    s
                } else {
                    match self.args.get(self.i) {
                        Some(s) => {
                            self.i += 1;
                            s.clone()
                        }
                        None => {
                            return Err(usage_err(format!(
                                "error: a value is required for '-{c}' but none was supplied"
                            )));
                        }
                    }
                };
                return Ok(Some(Tok::ShortVal(c, v)));
            }
            return Ok(Some(Tok::Short(c)));
        }
        if self.end_of_flags {
            return Ok(self.args.get(self.i).map(|s| {
                self.i += 1;
                Tok::Pos(s.clone())
            }));
        }
        let Some(a) = self.args.get(self.i) else {
            return Ok(None);
        };
        self.i += 1;
        if a == "--" {
            self.end_of_flags = true;
            return Ok(Some(Tok::End));
        }
        if let Some(l) = a.strip_prefix("--") {
            let (name, val) = match l.split_once('=') {
                Some((n, v)) => (n.to_string(), Some(v.to_string())),
                None => (l.to_string(), None),
            };
            return Ok(Some(Tok::Long(name, val)));
        }
        if let Some(s) = a.strip_prefix('-') {
            if s.is_empty() {
                return Err(usage_err("error: unexpected argument '-' found"));
            }
            if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                // Negative-looking number: treat as a positional so the
                // value parser can reject it with a proper message.
                return Ok(Some(Tok::Pos(a.clone())));
            }
            self.pending = s.chars().collect();
            return self.next(val_shorts);
        }
        Ok(Some(Tok::Pos(a.clone())))
    }
}
