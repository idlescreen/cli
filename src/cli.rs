// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Hand-rolled argument parser for `idlescreen` (replaces clap). All
//! legacy spellings survive as aliases so they show up in `--help`.

use std::fmt;
use std::io::Write;

/// Parse outcome categories, mirroring the clap `ErrorKind`s the caller
/// distinguishes (help/version are display paths, not failures).
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
    fn new(kind: ErrorKind, text: impl Into<String>) -> Self {
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

fn usage_err(msg: impl Into<String>) -> ParseError {
    ParseError::new(ErrorKind::Usage, msg)
}

fn help_err(text: impl Into<String>) -> ParseError {
    ParseError::new(ErrorKind::DisplayHelp, text)
}

pub struct Cli {
    /// Suppress confirmations and other non-essential output
    pub quiet: bool,
    pub cmd: Cmd,
}

/// Commands that do not need a running daemon are marked standalone.
impl Cmd {
    pub fn needs_daemon(&self) -> bool {
        !matches!(
            self,
            Cmd::Version { .. }
                | Cmd::About
                | Cmd::Doctor { .. }
                | Cmd::Clean { .. }
                | Cmd::Completion { .. }
                | Cmd::BugReport
                | Cmd::SelfUpdate { .. }
                | Cmd::Tui { .. }
                | Cmd::Restart
                | Cmd::Logs { .. }
                | Cmd::Config {
                    op: Some(ConfigOp::Path | ConfigOp::Edit | ConfigOp::Reset { .. }),
                    ..
                }
        )
    }
}

#[derive(Debug)]
pub enum Cmd {
    /// Show daemon state (running, idle, saver, inhibitors)
    Status {
        /// Machine-readable JSON output
        json: bool,
    },
    /// View or change daemon configuration
    Config {
        op: Option<ConfigOp>,
        /// Machine-readable JSON output
        json: bool,
    },
    /// Turn the idle screensaver on
    Enable,
    /// Turn the idle screensaver off
    Disable,
    /// Set or show the idle timeout in minutes (1–240)
    Timeout {
        minutes: Option<u32>,
        /// Machine-readable JSON output
        json: bool,
    },
    /// Show or change the active saver
    Saver {
        op: Option<SaverOp>,
        /// Machine-readable JSON output
        json: bool,
    },
    /// List installed savers
    List {
        /// Machine-readable JSON output
        json: bool,
    },
    /// List active idle inhibitors
    Inhibitors {
        /// Machine-readable JSON output
        json: bool,
    },
    /// Run a command while holding an idle inhibitor
    Inhibit {
        /// Reason recorded for the inhibitor
        reason: Option<String>,
        /// Command to run while idle is inhibited
        command: Vec<String>,
    },
    /// Activate the screensaver now (uses your configured saver)
    Start,
    /// Preview a saver fullscreen
    Preview {
        /// Saver name (see `idlescreen list`)
        name: String,
        /// Auto-stop after N seconds
        timeout: Option<u64>,
    },
    /// Stop the running preview or idle presentation
    Stop,
    /// FPS overlay: on, off, or status
    FpsOverlay { state: Option<OverlayState> },
    /// Render scale: 0.25–1.0, 'default', or status
    RenderScale { value: Option<String> },
    /// Interactive console panel
    Interactive,
    /// Restart the idle-daemon user service
    Restart,
    /// Show daemon logs (journalctl --user -u idle-daemon)
    Logs {
        /// Follow new entries (Ctrl-C to stop)
        follow: bool,
        /// Number of recent entries to show
        lines: u32,
    },
    /// Run diagnostics (--fix reloads the user service, --json prints a report)
    Doctor {
        /// Attempt to repair common issues
        fix: bool,
        /// Machine-readable JSON output
        json: bool,
    },
    /// Remove stale run state and log caches
    Clean {
        /// Show what would be removed without deleting anything
        dry_run: bool,
    },
    /// Print a shell completion script to stdout
    Completion { shell: CompletionShell },
    /// Print a sanitized diagnostics bundle for bug reports
    BugReport,
    /// Check for updates and upgrade installed IdleScreen packages
    SelfUpdate {
        /// Report update status without upgrading anything
        check: bool,
    },
    /// Launch the full-screen TUI
    Tui {
        /// Arguments forwarded to idle-tui
        args: Vec<String>,
    },
    /// Print CLI version
    Version {
        /// Extended info (same as `about`)
        long: bool,
        /// Machine-readable JSON output
        json: bool,
    },
    /// Print version plus project info
    About,
}

#[derive(Debug)]
pub enum ConfigOp {
    /// Show all configuration keys
    List,
    /// Show one configuration value
    Get { key: String },
    /// Set a configuration value
    Set { key: String, value: String },
    /// Print the configuration file path
    Path,
    /// Open the configuration file in $EDITOR
    Edit,
    /// Restore defaults by removing the configuration file
    Reset {
        /// Skip the confirmation prompt
        yes: bool,
    },
}

#[derive(Debug)]
pub enum SaverOp {
    /// Set the active saver (name, or 'random'/'none' for rotation)
    Set { name: String },
    /// List installed savers
    List,
}

#[derive(Debug, Clone, Copy)]
pub enum OverlayState {
    On,
    Off,
    Status,
}

/// Shells with generated completions (nu ships a static script).
#[derive(Debug, Clone, Copy)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
    Nushell,
    PowerShell,
    Elvish,
}

impl CompletionShell {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "bash" => Some(Self::Bash),
            "zsh" => Some(Self::Zsh),
            "fish" => Some(Self::Fish),
            "nushell" | "nu" => Some(Self::Nushell),
            "powershell" | "pwsh" | "power-shell" => Some(Self::PowerShell),
            "elvish" => Some(Self::Elvish),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Lexer: clap-compatible short-cluster / long / `--` / passthrough handling.
// ---------------------------------------------------------------------------

enum Tok {
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

struct Lexer<'a> {
    args: &'a [String],
    i: usize,
    /// Remaining chars of a short cluster being split (`-jq` → `j`,`q`).
    pending: Vec<char>,
    end_of_flags: bool,
}

impl<'a> Lexer<'a> {
    fn new(args: &'a [String]) -> Self {
        Self {
            args,
            i: 0,
            pending: Vec::new(),
            end_of_flags: false,
        }
    }

    /// `val_shorts`: short flags that consume a value (`-t 5` style).
    fn next(&mut self, val_shorts: &[char]) -> Result<Option<Tok>, ParseError> {
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

fn no_value(name: &str, val: Option<String>) -> Result<(), ParseError> {
    if val.is_some() {
        return Err(usage_err(format!(
            "error: unexpected value for '--{name}' found; no more were expected"
        )));
    }
    Ok(())
}

fn unknown_flag(name: &str) -> ParseError {
    usage_err(format!("error: unexpected argument '--{name}' found"))
}

fn unknown_short(c: char) -> ParseError {
    usage_err(format!("error: unexpected argument '-{c}' found"))
}

fn parse_u32(v: &str, what: &str) -> Result<u32, ParseError> {
    v.parse().map_err(|_| {
        usage_err(format!(
            "error: invalid value '{v}' for {what}: expected a number"
        ))
    })
}

fn parse_u64(v: &str, what: &str) -> Result<u64, ParseError> {
    v.parse().map_err(|_| {
        usage_err(format!(
            "error: invalid value '{v}' for {what}: expected a number"
        ))
    })
}

// ---------------------------------------------------------------------------
// Top-level parse
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Subcommand dispatch
// ---------------------------------------------------------------------------

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
        "status" => {
            let json = flags_json(&mut lx, quiet, name)?;
            Ok(Cmd::Status { json })
        }
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
        "list" => {
            let json = flags_json(&mut lx, quiet, name)?;
            Ok(Cmd::List { json })
        }
        "inhibitors" => {
            let json = flags_json(&mut lx, quiet, name)?;
            Ok(Cmd::Inhibitors { json })
        }
        "timeout" => {
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
                            return Err(usage_err(format!(
                                "error: unexpected argument '{s}' found"
                            )));
                        }
                        minutes = Some(parse_u32(&s, "'<minutes>'")?);
                    }
                    Tok::End => {}
                }
            }
            Ok(Cmd::Timeout { minutes, json })
        }
        "preview" => {
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
                            let v = flag_value(&mut lx, &n, v)?;
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
                            return Err(usage_err(format!(
                                "error: unexpected argument '{s}' found"
                            )));
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
        "config" => parse_config(&mut lx, quiet),
        "saver" => parse_saver(&mut lx, quiet),
        "inhibit" => parse_inhibit(args, quiet),
        "fps-overlay" => {
            let mut state = None;
            while let Some(t) = lx.next(&[])? {
                match t {
                    Tok::Short('q') => *quiet = true,
                    Tok::Short('h') => return Err(help_err(help_for(name))),
                    Tok::Short(c) => return Err(unknown_short(c)),
                    Tok::Long(n, v) => {
                        no_value(&n, v)?;
                        return Err(unknown_flag(&n));
                    }
                    Tok::Pos(s) => {
                        if state.is_some() {
                            return Err(usage_err(format!(
                                "error: unexpected argument '{s}' found"
                            )));
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
                    Tok::ShortVal(c, _) => return Err(unknown_short(c)),
                    Tok::End => {}
                }
            }
            Ok(Cmd::FpsOverlay { state })
        }
        "render-scale" => {
            let mut value = None;
            while let Some(t) = lx.next(&[])? {
                match t {
                    Tok::Short('q') => *quiet = true,
                    Tok::Short('h') => return Err(help_err(help_for(name))),
                    Tok::Short(c) => return Err(unknown_short(c)),
                    Tok::Long(n, v) => {
                        no_value(&n, v)?;
                        return Err(unknown_flag(&n));
                    }
                    Tok::Pos(s) => {
                        if value.is_some() {
                            return Err(usage_err(format!(
                                "error: unexpected argument '{s}' found"
                            )));
                        }
                        value = Some(s);
                    }
                    Tok::ShortVal(c, _) => return Err(unknown_short(c)),
                    Tok::End => {}
                }
            }
            Ok(Cmd::RenderScale { value })
        }
        "logs" => {
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
                            let v = flag_value(&mut lx, &n, v)?;
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
        "doctor" => {
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
        "clean" => {
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
        "completion" => {
            let mut shell = None;
            while let Some(t) = lx.next(&[])? {
                match t {
                    Tok::Short('q') => *quiet = true,
                    Tok::Short('h') => return Err(help_err(help_for(name))),
                    Tok::Short(c) => return Err(unknown_short(c)),
                    Tok::Long(n, v) => {
                        no_value(&n, v)?;
                        return Err(unknown_flag(&n));
                    }
                    Tok::Pos(s) => {
                        if shell.is_some() {
                            return Err(usage_err(format!(
                                "error: unexpected argument '{s}' found"
                            )));
                        }
                        shell = Some(CompletionShell::parse(&s).ok_or_else(|| {
                            usage_err(format!(
                                "error: invalid value '{s}' for '<shell>' [possible values: bash, zsh, fish, nushell, powershell, elvish]"
                            ))
                        })?);
                    }
                    Tok::ShortVal(c, _) => return Err(unknown_short(c)),
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
        "self-update" => {
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
        "tui" => {
            // Pure passthrough: everything after `tui` goes to idle-tui.
            Ok(Cmd::Tui {
                args: args.to_vec(),
            })
        }
        "version" => {
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
        _ => unreachable!("canonical names are exhaustive"),
    }
}

// ---------------------------------------------------------------------------
// Shared flag helpers
// ---------------------------------------------------------------------------

/// A `--flag value` or `--flag=value` pair.
fn flag_value(
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
fn flags_none(lx: &mut Lexer<'_>, quiet: &mut bool, name: &'static str) -> Result<(), ParseError> {
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
fn flags_json(
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

/// `config [op]` — `--json` is global across the config subtree.
fn parse_config(lx: &mut Lexer<'_>, quiet: &mut bool) -> Result<Cmd, ParseError> {
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

fn parse_config_op(
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
fn parse_saver(lx: &mut Lexer<'_>, quiet: &mut bool) -> Result<Cmd, ParseError> {
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
fn finish_nested(
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

fn required_pos(lx: &mut Lexer<'_>, what: &str, ctx: &str) -> Result<String, ParseError> {
    match lx.next(&[])? {
        Some(Tok::Pos(s)) => Ok(s),
        _ => Err(usage_err(format!(
            "error: the following required arguments were not provided:\n  {what}\n\nUsage: idlescreen {ctx} {what}\n\nFor more information, try '--help'."
        ))),
    }
}

/// `inhibit [-r reason] <command...>` — flags parsed until the first
/// positional; everything after (hyphens included) is the child argv,
/// captured verbatim.
fn parse_inhibit(args: &[String], quiet: &mut bool) -> Result<Cmd, ParseError> {
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

// ---------------------------------------------------------------------------
// Help text
// ---------------------------------------------------------------------------

fn help_top() -> String {
    "Control the IdleScreen Wayland screensaver daemon.\n\
     Run `idlescreen <command> --help` for subcommand details.\n\
     \n\
     Usage: idlescreen [OPTIONS] <COMMAND>\n\
     \n\
     Commands:\n\
     \x20 status        Show daemon state (running, idle, saver, inhibitors) [alias: st]\n\
     \x20 config        View or change daemon configuration [alias: cfg]\n\
     \x20 enable        Turn the idle screensaver on [alias: on]\n\
     \x20 disable       Turn the idle screensaver off [alias: off]\n\
     \x20 timeout       Set or show the idle timeout in minutes (1-240) [alias: t]\n\
     \x20 saver         Show or change the active saver [alias: sv]\n\
     \x20 list          List installed savers [alias: ls]\n\
     \x20 inhibitors    List active idle inhibitors [alias: inhib]\n\
     \x20 inhibit       Run a command while holding an idle inhibitor [alias: hold]\n\
     \x20 start         Activate the screensaver now [alias: activate]\n\
     \x20 preview       Preview a saver fullscreen [alias: p]\n\
     \x20 stop          Stop the running preview or idle presentation [alias: x]\n\
     \x20 fps-overlay   FPS overlay: on, off, or status [alias: fps]\n\
     \x20 render-scale  Render scale: 0.25-1.0, 'default', or status [alias: scale]\n\
     \x20 interactive   Interactive console panel [alias: i]\n\
     \x20 restart       Restart the idle-daemon user service [alias: rs]\n\
     \x20 logs          Show daemon logs [alias: log]\n\
     \x20 doctor        Run diagnostics [alias: doc]\n\
     \x20 clean         Remove stale run state and log caches [alias: cl]\n\
     \x20 completion    Print a shell completion script to stdout [alias: comp]\n\
     \x20 bug-report    Print a sanitized diagnostics bundle [alias: bug]\n\
     \x20 self-update   Check for updates and upgrade packages [aliases: update, upgrade]\n\
     \x20 tui           Launch the full-screen TUI [alias: ui]\n\
     \x20 version       Print CLI version [alias: v]\n\
     \x20 about         Print version plus project info [alias: info]\n\
     \x20 help          Print this message or the help of the given subcommand(s)\n\
     \n\
     Options:\n\
     \x20 -q, --quiet     Suppress confirmations and other non-essential output\n\
     \x20 -h, --help      Print help\n\
     \x20 -V, --version   Print version"
        .to_string()
}

fn help_for(name: &str) -> String {
    let (about, usage, opts) = match name {
        "status" => (
            "Show daemon state (running, idle, saver, inhibitors)",
            "idlescreen status [OPTIONS]",
            "  -j, --json   Machine-readable JSON output",
        ),
        "config" => (
            "View or change daemon configuration",
            "idlescreen config [OPTIONS] [COMMAND]",
            "  -j, --json   Machine-readable JSON output\n\n\
             Commands:\n  list   Show all configuration keys\n  \
             get    Show one configuration value: get <key>\n  \
             set    Set a configuration value: set <key> <value>\n  \
             path   Print the configuration file path\n  \
             edit   Open the configuration file in $EDITOR\n  \
             reset  Restore defaults: reset [-y|--yes]",
        ),
        "saver" => (
            "Show or change the active saver",
            "idlescreen saver [OPTIONS] [COMMAND]",
            "  -j, --json   Machine-readable JSON output\n\n\
             Commands:\n  set <name>  Set the active saver ('random'/'none' for rotation)\n  \
             list        List installed savers",
        ),
        "timeout" => (
            "Set or show the idle timeout in minutes (1-240)",
            "idlescreen timeout [OPTIONS] [minutes]",
            "  -j, --json   Machine-readable JSON output",
        ),
        "list" | "inhibitors" => (
            "List installed savers / active idle inhibitors",
            "idlescreen list [OPTIONS]",
            "  -j, --json   Machine-readable JSON output",
        ),
        "inhibit" => (
            "Run a command while holding an idle inhibitor",
            "idlescreen inhibit [OPTIONS] <command>...",
            "  -r, --reason <reason>  Reason recorded for the inhibitor",
        ),
        "preview" => (
            "Preview a saver fullscreen",
            "idlescreen preview [OPTIONS] <name>",
            "  -t, --timeout <seconds>  Auto-stop after N seconds",
        ),
        "fps-overlay" => (
            "FPS overlay: on, off, or status",
            "idlescreen fps-overlay [on|off|status]",
            "",
        ),
        "render-scale" => (
            "Render scale: 0.25-1.0, 'default', or status",
            "idlescreen render-scale [value]",
            "",
        ),
        "logs" => (
            "Show daemon logs (journalctl --user -u idle-daemon)",
            "idlescreen logs [OPTIONS]",
            "  -f, --follow       Follow new entries (Ctrl-C to stop)\n  \
             -n, --lines <n>    Number of recent entries to show [default: 100]",
        ),
        "doctor" => (
            "Run diagnostics",
            "idlescreen doctor [OPTIONS]",
            "  -f, --fix    Attempt to repair common issues\n  \
             -j, --json   Machine-readable JSON output",
        ),
        "clean" => (
            "Remove stale run state and log caches",
            "idlescreen clean [OPTIONS]",
            "  -n, --dry-run  Show what would be removed without deleting anything",
        ),
        "completion" => (
            "Print a shell completion script to stdout",
            "idlescreen completion <shell>",
            "  shell: bash, zsh, fish, nushell (nu), powershell (pwsh), elvish",
        ),
        "self-update" => (
            "Check for updates and upgrade installed IdleScreen packages",
            "idlescreen self-update [OPTIONS]",
            "  -c, --check  Report update status without upgrading anything",
        ),
        "tui" => (
            "Launch the full-screen TUI",
            "idlescreen tui [args]...",
            "  args are forwarded to idle-tui",
        ),
        "version" => (
            "Print CLI version",
            "idlescreen version [OPTIONS]",
            "  -l, --long   Extended info (same as `about`)\n  \
             -j, --json   Machine-readable JSON output",
        ),
        _ => ("Control the IdleScreen daemon", "idlescreen <COMMAND>", ""),
    };
    let mut s = format!("{about}\n\nUsage: {usage}\n\nOptions:");
    if !opts.is_empty() {
        s.push('\n');
        s.push_str(opts);
    }
    s.push_str("\n  -q, --quiet   Suppress confirmations and other non-essential output\n  -h, --help    Print help");
    s
}
