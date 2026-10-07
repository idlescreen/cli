// Tests for `doctor_hw`. Split out via `#[path]` in `doctor_hw.rs` to keep
// that page inside the 256-line ceiling; name resolution is unchanged because
// this is still a child module of `doctor_hw`.

use super::*;

#[test]
fn test_mock_nodes() {
    let temp = std::env::temp_dir().join("test_doctor_nodes");
    let _ = fs::create_dir_all(&temp);
    use crate::doctor::checks::Severity::{Ok as OkS, Warn as WarnS};
    assert_eq!(check_gpu_at(&temp).severity, WarnS);
    assert_eq!(check_direct_scanout_at(&temp).severity, WarnS);
    let _ = fs::write(temp.join("renderD128"), b"");
    let _ = fs::write(temp.join("card0"), b"");
    assert_eq!(check_gpu_at(&temp).severity, OkS);
    assert_eq!(check_direct_scanout_at(&temp).severity, OkS);
    assert_eq!(check_dmabuf_at(&temp, &temp).severity, OkS);
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
    let atom = cpu.join("types/intel_atom_0");
    let _ = fs::create_dir_all(&atom);
    let _ = fs::write(atom.join("cpulist"), b"4-7\n");
    assert!(check_ecores_at(&cpu).detail.contains("4 efficiency cores"));
    let _ = fs::remove_dir_all(&cpu);
}

#[test]
fn test_check_hardware_real() {
    let results = check_hardware();
    assert_eq!(results.len(), 6);
    assert!(results.iter().all(|r| r.passed()));
}
