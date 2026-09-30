// SPDX-License-Identifier: MIT

//! Hardware capabilities and driver feature detection for `idlescreen doctor`.

use std::fs;
use std::path::Path;

use crate::doctor::checks::{CheckResult, ok, warn};

fn has_file(dir: &Path, prefix: &str) -> bool {
    fs::read_dir(dir).is_ok_and(|entries| {
        entries
            .filter_map(std::result::Result::ok)
            .any(|e| e.file_name().to_string_lossy().starts_with(prefix))
    })
}

pub fn check_gpu_at(dri_dir: &Path) -> CheckResult {
    if dri_dir.join("renderD128").exists() || has_file(dri_dir, "renderD") {
        ok("GPU Acceleration", "DRI render node accessible")
    } else {
        warn(
            "GPU Acceleration",
            "no DRI render node found; software rasterizer",
        )
    }
}

pub fn check_gpu() -> CheckResult {
    check_gpu_at(Path::new("/dev/dri"))
}

pub fn check_dmabuf_at(heap_dir: &Path, dri_dir: &Path) -> CheckResult {
    if heap_dir.exists() || dri_dir.join("renderD128").exists() {
        ok("DMA-BUF Zero-Copy", "kernel DMA-BUF sharing supported")
    } else {
        warn(
            "DMA-BUF Zero-Copy",
            "DMA-BUF heap not found; scanout unavail",
        )
    }
}

pub fn check_dmabuf() -> CheckResult {
    check_dmabuf_at(Path::new("/dev/dma_heap"), Path::new("/dev/dri"))
}

pub fn check_direct_scanout_at(dri_dir: &Path) -> CheckResult {
    if has_file(dri_dir, "card") {
        ok(
            "Direct Scanout",
            "KMS overlay planes accessible via card node",
        )
    } else {
        warn("Direct Scanout", "no KMS primary card node accessible")
    }
}

pub fn check_direct_scanout() -> CheckResult {
    check_direct_scanout_at(Path::new("/dev/dri"))
}

pub fn check_vrr_at(drm_dir: &Path) -> CheckResult {
    let mut any_vrr = false;
    let mut any_connector = false;
    if let Ok(entries) = fs::read_dir(drm_dir) {
        for entry in entries.flatten() {
            let vrr_path = entry.path().join("vrr_capable");
            if vrr_path.exists() {
                any_connector = true;
                if fs::read_to_string(&vrr_path).is_ok_and(|v| v.trim() == "1") {
                    any_vrr = true;
                    break;
                }
            }
        }
    }
    if any_vrr {
        ok(
            "VRR / Adaptive Sync",
            "display supports adaptive sync (vrr_capable=1)",
        )
    } else if any_connector {
        ok(
            "VRR / Adaptive Sync",
            "KMS VRR supported (current panel fixed refresh)",
        )
    } else {
        warn("VRR / Adaptive Sync", "no VRR-capable connectors detected")
    }
}

pub fn check_vrr() -> CheckResult {
    check_vrr_at(Path::new("/sys/class/drm"))
}

pub fn check_hdr_at(drm_dir: &Path) -> CheckResult {
    let any_hdr = fs::read_dir(drm_dir).is_ok_and(|entries| {
        entries
            .flatten()
            .any(|e| e.path().join("hdr_output_metadata").exists())
    });
    if any_hdr {
        ok(
            "HDR10 / Deep Color",
            "display HDR output metadata supported",
        )
    } else {
        ok(
            "HDR10 / Deep Color",
            "standard dynamic range (sRGB 8-bit color depth)",
        )
    }
}

pub fn check_hdr() -> CheckResult {
    check_hdr_at(Path::new("/sys/class/drm"))
}

pub fn check_ecores_at(cpu_dir: &Path) -> CheckResult {
    let mut count = 0usize;
    if let Ok(entries) = fs::read_dir(cpu_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let s = name.to_string_lossy();
            if s.starts_with("cpu") && s[3..].chars().all(|c| c.is_ascii_digit()) {
                let p = entry.path().join("topology/core_type");
                if fs::read_to_string(p).is_ok_and(|v| v.trim().eq_ignore_ascii_case("atom")) {
                    count = count.saturating_add(1);
                }
            }
        }
    }
    if count > 0 {
        ok(
            "CPU Affinity (E-Cores)",
            format!("{count} efficiency cores identified"),
        )
    } else if cpu_dir.exists() {
        ok(
            "CPU Affinity (E-Cores)",
            "homogeneous CPU topology (all performance cores)",
        )
    } else {
        warn(
            "CPU Affinity (E-Cores)",
            "unable to read /sys/devices/system/cpu",
        )
    }
}

pub fn check_ecores() -> CheckResult {
    check_ecores_at(Path::new("/sys/devices/system/cpu"))
}

/// Runs all hardware probes for `idlescreen doctor`.
pub fn check_hardware() -> Vec<CheckResult> {
    vec![
        check_gpu(),
        check_dmabuf(),
        check_direct_scanout(),
        check_vrr(),
        check_hdr(),
        check_ecores(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_nodes() {
        let temp = std::env::temp_dir().join("test_doctor_nodes");
        let _ = fs::create_dir_all(&temp);
        assert!(!check_gpu_at(&temp).detail.contains("accessible"));
        assert!(!check_direct_scanout_at(&temp).detail.contains("accessible"));
        let _ = fs::write(temp.join("renderD128"), b"");
        let _ = fs::write(temp.join("card0"), b"");
        assert!(check_gpu_at(&temp).detail.contains("accessible"));
        assert!(check_direct_scanout_at(&temp).detail.contains("accessible"));
        assert!(check_dmabuf_at(&temp, &temp).detail.contains("supported"));
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_vrr_hdr_and_ecores_mock() {
        let drm = std::env::temp_dir().join("test_doctor_drm2");
        let conn = drm.join("card0-DP-1");
        let _ = fs::create_dir_all(&conn);
        let _ = fs::write(conn.join("vrr_capable"), b"1\n");
        let _ = fs::write(conn.join("hdr_output_metadata"), b"1\n");
        assert!(check_vrr_at(&drm).detail.contains("adaptive sync"));
        assert!(check_hdr_at(&drm).detail.contains("HDR output metadata"));
        let _ = fs::remove_dir_all(&drm);

        let cpu = std::env::temp_dir().join("test_doctor_cpu2");
        let topo = cpu.join("cpu0/topology");
        let _ = fs::create_dir_all(&topo);
        let _ = fs::write(topo.join("core_type"), b"Atom\n");
        assert!(check_ecores_at(&cpu).detail.contains("1 efficiency cores"));
        let _ = fs::remove_dir_all(&cpu);
    }

    #[test]
    fn test_check_hardware_real() {
        let results = check_hardware();
        assert_eq!(results.len(), 6);
        for r in &results {
            assert!(r.passed());
        }
    }
}
