// SPDX-License-Identifier: MIT

use super::checks::{CheckResult, fail, ok, warn};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn check_fonts() -> CheckResult {
    if font_check_via_fc_list() {
        ok("System Fonts", "monospace font found")
    } else {
        fail(
            "System Fonts",
            "monospace font missing — install fonts-dejavu-core or another mono font",
        )
    }
}

/// cgroup v2 delegation — the runner's budget enforcement needs `cpu` (and
/// ideally `memory`) controllers in the user's subtree.
pub fn check_cgroup() -> CheckResult {
    let subtree = Path::new("/sys/fs/cgroup/user.slice/cgroup.subtree_control");
    let controllers = Path::new("/sys/fs/cgroup/cgroup.controllers");
    if !controllers.exists() {
        return warn(
            "Cgroup Budget",
            "cgroup v2 not mounted — saver CPU/memory limits inactive",
        );
    }
    let delegated = fs::read_to_string(controllers).unwrap_or_default();
    let subtree_ctrls = fs::read_to_string(subtree).unwrap_or_default();
    let has_cpu = delegated.contains("cpu") || subtree_ctrls.contains("cpu");
    let has_mem = delegated.contains("memory") || subtree_ctrls.contains("memory");
    match (has_cpu, has_mem) {
        (true, true) => ok("Cgroup Budget", "cpu+memory controllers available"),
        (true, false) => warn(
            "Cgroup Budget",
            "cpu only — memory.max delegation unavailable (saver OOM uncapped)",
        ),
        _ => warn(
            "Cgroup Budget",
            "no delegated controllers — saver resource limits inactive",
        )
        .with_fix("delegate cpu/memory: systemctl edit --user or cgroup2 delegation"),
    }
}

fn font_check_via_fc_list() -> bool {
    let output = Command::new("fc-list").args([":mono"]).output();
    match output {
        Ok(out) => out.status.success() && !out.stdout.is_empty(),
        Err(_) => {
            let common_dirs = ["/usr/share/fonts", "/usr/local/share/fonts"];
            common_dirs.iter().any(|dir| PathBuf::from(dir).exists())
        }
    }
}

/// Dirty-rectangle diffing SIMD acceleration check (AVX2 on x86_64, NEON on aarch64).
pub fn check_simd() -> CheckResult {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return ok("SIMD Acceleration", "AVX2 dirty-rect diffing available");
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        return ok("SIMD Acceleration", "NEON dirty-rect diffing available");
    }

    warn(
        "SIMD Acceleration",
        "no AVX2/NEON detected; dirty-rect diffing using scalar fallback",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_simd() {
        let res = check_simd();
        assert_eq!(res.name, "SIMD Acceleration");
        assert!(res.passed());
    }

    #[test]
    fn test_font_check() {
        let res = check_fonts();
        assert_eq!(res.name, "System Fonts");
    }
}
