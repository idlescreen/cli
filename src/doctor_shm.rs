// SPDX-License-Identifier: MIT
// perf: T3 · metric: touches the filesystem; dominated by syscall latency, not by this page's logic · check: test
//! /dev/shm probe for `doctor`.
//!
//! Split out of `doctor_fs.rs`, which was sitting exactly on the 256-line
//! ceiling and had no room for a label or for any change at all.

use super::doctor_checks::{CheckResult, fail, ok, warn};
use std::fs;
use std::path::PathBuf;

/// /dev/shm probe — a missing dir is a WARN (memfd fallback), not a failure.
pub fn check_shm_permissions() -> CheckResult {
    let shm_dir = PathBuf::from("/dev/shm");
    if shm_dir.exists() {
        let test_file = shm_dir.join(format!(".idle-doctor-test-{}", std::process::id()));
        if fs::write(&test_file, b"test").is_ok() {
            let _ = fs::remove_file(&test_file);
            ok("Shared Memory", "/dev/shm writable")
        } else {
            fail("Shared Memory", "/dev/shm permission denied")
                .with_fix("check /dev/shm mount and permissions")
        }
    } else {
        warn("Shared Memory", "/dev/shm absent — memfd fallback in use")
    }
}

#[cfg(test)]
mod tests {
    use super::check_shm_permissions;

    /// The probe must always return a verdict, never panic, and never
    /// leave its scratch file behind. It is the one check that writes
    /// to a real path, so that is the property worth pinning.
    #[test]
    fn shm_probe_returns_a_verdict_and_cleans_up() {
        let before = std::fs::read_dir("/dev/shm")
            .map(|d| d.count())
            .unwrap_or(0);
        let result = check_shm_permissions();
        let after = std::fs::read_dir("/dev/shm")
            .map(|d| d.count())
            .unwrap_or(0);
        assert_eq!(before, after, "shm probe left a scratch file behind");
        assert!(!result.name.is_empty());
    }
}
