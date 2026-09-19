// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Minimal logging replacing `tracing`/`tracing-subscriber`/
//! `tracing-journald`: level-filtered `error!`..`trace!` macros writing to
//! stderr, controlled by `RUST_LOG` (same simple grammar: a bare level or
//! `target=level` list — we take the max enabled level). With the
//! `journald` feature, messages are also sent to the systemd journal via a
//! datagram to `/run/systemd/journal/socket` (the whole sd-journal protocol).

use std::io::Write;
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Level {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl Level {
    fn from_name(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "error" => Some(Self::Error),
            "warn" | "warning" => Some(Self::Warn),
            "info" => Some(Self::Info),
            "debug" => Some(Self::Debug),
            "trace" => Some(Self::Trace),
            _ => None,
        }
    }

    #[cfg(feature = "journald")]
    fn priority(self) -> u8 {
        // syslog priorities used by the journal: 3=err .. 7=debug.
        match self {
            Self::Error => 3,
            Self::Warn => 4,
            Self::Info => 6,
            Self::Debug | Self::Trace => 7,
        }
    }
}

/// Enabled threshold; `off` (0) disables everything. Default: warn.
static ENABLED: AtomicU8 = AtomicU8::new(Level::Warn as u8);

/// Install the filter from `RUST_LOG`; absent/unparseable keeps `warn`.
/// Accepts `RUST_LOG=debug`, `RUST_LOG=idle=debug,zbus=warn`, `RUST_LOG=off`.
pub fn init() {
    let Ok(spec) = std::env::var("RUST_LOG") else {
        return;
    };
    let mut max = 0u8;
    for part in spec.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let level = part
            .rsplit_once('=')
            .map_or_else(|| Level::from_name(part), |(_, l)| Level::from_name(l));
        if let Some(l) = level {
            max = max.max(l as u8);
        }
    }
    ENABLED.store(max, Ordering::Relaxed);
}

pub fn enabled(level: Level) -> bool {
    level as u8 <= ENABLED.load(Ordering::Relaxed)
}

/// Emit one record (stderr always; journal too under the feature).
pub fn emit(level: Level, target: &str, msg: std::fmt::Arguments<'_>) {
    if !enabled(level) {
        return;
    }
    let text = msg.to_string();
    let name = match level {
        Level::Error => "ERROR",
        Level::Warn => "WARN",
        Level::Info => "INFO",
        Level::Debug => "DEBUG",
        Level::Trace => "TRACE",
    };
    let _ = writeln!(std::io::stderr().lock(), "{name} {target}: {text}");
    #[cfg(feature = "journald")]
    journald_send(level.priority(), &text);
}

/// sd-journal over `/run/systemd/journal/socket`: newline-separated
/// `KEY=value` fields in a single datagram. Best-effort; failures ignored.
#[cfg(feature = "journald")]
fn journald_send(priority: u8, msg: &str) {
    use std::os::unix::net::UnixDatagram;
    let Ok(sock) = UnixDatagram::unbound() else {
        return;
    };
    let payload = format!(
        "PRIORITY={priority}\nMESSAGE={}\nSYSLOG_IDENTIFIER=idle-cli\n",
        msg.replace('\n', " ")
    );
    let _ = sock.send_to(payload.as_bytes(), "/run/systemd/journal/socket");
}

// `#[macro_export]` rather than `pub(crate) use`: `warn` is a builtin
// attribute name and cannot be re-exported through a `use` path (E0659).
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::emit($crate::log::Level::Error, module_path!(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log::emit($crate::log::Level::Warn, module_path!(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::emit($crate::log::Level::Info, module_path!(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::emit($crate::log::Level::Debug, module_path!(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log::emit($crate::log::Level::Trace, module_path!(), format_args!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // ENABLED is process-global and init() reads the process-global
    // RUST_LOG — keep every env-dependent assertion inside this one test
    // so parallel tests can't interleave on either.
    #[test]
    fn level_filtering_follows_rust_log() {
        // SAFETY: only this test in the crate reads/writes RUST_LOG.
        unsafe { std::env::set_var("RUST_LOG", "debug") };
        init();
        assert!(enabled(Level::Error));
        assert!(enabled(Level::Debug));
        assert!(!enabled(Level::Trace));

        // target=level list: max enabled level wins.
        unsafe { std::env::set_var("RUST_LOG", "idle_cli=error,zbus=warn") };
        init();
        assert!(enabled(Level::Warn));
        assert!(!enabled(Level::Info));

        unsafe { std::env::set_var("RUST_LOG", "trace") };
        init();
        assert!(enabled(Level::Trace));

        unsafe { std::env::set_var("RUST_LOG", "bogus-garbage") };
        init();
        assert!(!enabled(Level::Error), "unparseable spec mutes all");

        unsafe { std::env::set_var("RUST_LOG", "off") };
        init();
        assert!(!enabled(Level::Error));

        // Restore the warn default for other tests/binaries.
        unsafe { std::env::remove_var("RUST_LOG") };
        ENABLED.store(Level::Warn as u8, Ordering::Relaxed);
    }

    #[test]
    fn emit_does_not_panic() {
        // Writes to real stderr when enabled; gated path returns early
        // otherwise. Either way must not panic.
        emit(Level::Error, "test", format_args!("smoke {}", 1));
    }
}
