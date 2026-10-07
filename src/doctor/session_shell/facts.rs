// SPDX-License-Identifier: MIT

//! Gathers the observable facts about a session-shell integration.
//!
//! Everything here is I/O; the judgement lives in [`classify`] so it can be
//! unit-tested without a filesystem.

use std::path::PathBuf;

/// Where the interception shim is installed. Overridable so tests never touch
/// the real `/usr/local/bin`.
pub fn shim_path() -> PathBuf {
    std::env::var_os("IDLESCREEN_SESSION_SHIM")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/usr/local/bin/omarchy-launch-screensaver"))
}

/// Is `name` an executable reachable on `PATH`?
pub fn path_has(name: &str) -> bool {
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&path).any(|dir| {
        let candidate = dir.join(name);
        candidate.is_file()
    })
}

/// Everything the integration check reasons about.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IntegrationFacts {
    /// A session shell that owns idle timing and lock is installed.
    pub shell_present: bool,
    /// The interception shim is on disk.
    pub shim_present: bool,
    /// The `idlescreen` router the shim execs resolves on `PATH`.
    pub router_present: bool,
    /// `idle_enabled` from config; `None` when unset or unreadable.
    pub idle_enabled: Option<bool>,
    /// Live idle inhibitors, which silently suppress the screensaver.
    /// `None` when the daemon could not be reached.
    pub inhibitor_count: Option<usize>,
}

pub fn gather() -> IntegrationFacts {
    IntegrationFacts {
        shell_present: path_has("omarchy-shell"),
        shim_present: shim_path().is_file(),
        router_present: path_has("idlescreen") || path_has("idle-cli"),
        idle_enabled: read_idle_enabled(),
        inhibitor_count: read_inhibitor_count(),
    }
}

/// Parse `idle_enabled` out of the user config without pulling in the
/// daemon's config parser.
pub fn read_idle_enabled() -> Option<bool> {
    let path = config_path()?;
    let body = std::fs::read_to_string(path).ok()?;
    for line in body.lines() {
        let line = line.trim();
        // Skip the leading key match but honour `key: value` with any indent.
        let Some(rest) = line.strip_prefix("idle_enabled:") else {
            continue;
        };
        return match rest.trim() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        };
    }
    None
}

fn config_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join("idlescreen").join("config.yaml"))
}

/// Live inhibitors from the daemon, or `None` when the daemon cannot be
/// reached. `None` is deliberately distinct from `Some(0)`: an unreachable
/// daemon must not be reported as a session with nothing suppressing it.
fn read_inhibitor_count() -> Option<usize> {
    use idle_dbus::TranceClient;
    let client = TranceClient::connect().ok()?;
    client.list_inhibitors().ok().map(|v| v.len())
}
