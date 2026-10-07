// SPDX-License-Identifier: MIT

//! Gathers the observable facts about a session-shell integration.
//!
//! Everything here is I/O; the judgement lives in [`classify`] so it can be
//! unit-tested without a filesystem.

use std::path::{Path, PathBuf};

/// Where the interception shim is installed. Overridable so tests never touch
/// the real `/usr/local/bin`.
pub fn shim_path() -> PathBuf {
    std::env::var_os("IDLESCREEN_SESSION_SHIM")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/usr/local/bin").join(SHIM_NAME))
}

/// Is `name` an executable reachable on `PATH`?
pub fn path_has(name: &str) -> bool {
    resolve_on_path(name).is_some()
}

/// The first `PATH` entry holding a file called `name`.
///
/// Deliberately the same predicate [`path_has`] used before this split, so the
/// "is it reachable" fact and the "what does it resolve to" fact can never
/// disagree about the same `PATH`.
fn resolve_on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
}

/// What the name lookup for `omarchy-launch-screensaver` actually lands on.
///
/// The shim works by winning that lookup. Checking that the shim *exists* is
/// not enough: the whole scheme silently does nothing if some other directory
/// earlier on `PATH` answers to the same name, which is exactly what happens
/// if a login environment reorders `PATH` ahead of `/usr/local/bin`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Resolution {
    /// Nothing on this `PATH` is called `omarchy-launch-screensaver`.
    #[default]
    Absent,
    /// The lookup lands on our shim. Interception is live.
    Shim,
    /// The lookup lands on a different file. Our shim is inert.
    Shadowed(PathBuf),
}

/// Resolve the shim name against `PATH`, relative to [`shim_path`].
pub fn resolve_shim() -> Resolution {
    let found = match resolve_on_path(SHIM_NAME) {
        Some(found) => found,
        None => return Resolution::Absent,
    };
    if same_file(&found, &shim_path()) {
        Resolution::Shim
    } else {
        Resolution::Shadowed(found)
    }
}

/// The command name the session shell invokes to launch its screensaver.
/// This is the one seam the whole integration hangs off.
pub const SHIM_NAME: &str = "omarchy-launch-screensaver";

/// Compare two paths, tolerating symlinks and a non-canonical `PATH`.
///
/// Falls back to plain equality when canonicalization fails (a broken or
/// permission-denied entry must not be reported as a match).
fn same_file(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}

/// Everything the integration check reasons about.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IntegrationFacts {
    /// A session shell that owns idle timing and lock is installed.
    pub shell_present: bool,
    /// The interception shim is on disk.
    pub shim_present: bool,
    /// What `omarchy-launch-screensaver` actually resolves to on `PATH`.
    /// A shim that is on disk but loses the name lookup is inert, and the
    /// distinction is invisible from every other field here.
    pub shim_resolution: Resolution,
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
        shim_resolution: resolve_shim(),
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
