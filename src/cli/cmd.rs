// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Cli/Cmd/ConfigOp/SaverOp/OverlayState/CompletionShell — the parse
//! result types every other cli module builds.

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
    pub(crate) fn parse(s: &str) -> Option<Self> {
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
