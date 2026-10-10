// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Daemon service lifecycle: `restart` and `logs` across all init systems.
//! Works with or without a live daemon.

use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use crate::err::{Context, Result, bail};
use crate::self_update::backend::which;
use idle_dbus::daemon_available;
use idle_dbus::service::{InitSystem, detect_init_system, restart_daemon_service};

/// Current unit plus the legacy name so old installs still show logs.
const UNITS: &[&str] = &["idle-daemon", "trance-daemon"];

pub fn handle_restart() -> Result<()> {
    if let Err(e) = restart_daemon_service() {
        bail!("failed to restart daemon service: {e}");
    }
    // The unit restart returns before the D-Bus name is claimed; wait for it.
    for _ in 0..30 {
        if daemon_available() {
            if !crate::quiet() {
                println!("Daemon restarted.");
            }
            return Ok(());
        }
        sleep(Duration::from_millis(100));
    }
    println!("Restart issued; idle-daemon is not reachable on D-Bus yet.");
    Ok(())
}

pub fn handle_logs(follow: bool, lines: u32) -> Result<()> {
    let init = detect_init_system();
    if init == InitSystem::Systemd && which("journalctl") {
        let mut args = vec![
            "--user".to_string(),
            "--output".to_string(),
            "short".to_string(),
        ];
        for unit in UNITS {
            args.push("-u".to_string());
            args.push(format!("{unit}.service"));
        }
        args.push("-n".to_string());
        args.push(lines.to_string());
        if follow {
            args.push("-f".to_string());
        }
        let status = Command::new("journalctl")
            .args(&args)
            .status()
            .context("failed to run journalctl")?;
        if status.success() {
            return Ok(());
        }
        std::process::exit(status.code().unwrap_or(1));
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let state_log = std::path::PathBuf::from(home)
        .join(".local")
        .join("state")
        .join("idlescreen")
        .join("idle-daemon.log");

    if state_log.exists() {
        if which("tail") {
            let mut args = vec!["-n".to_string(), lines.to_string()];
            if follow {
                args.push("-f".to_string());
            }
            args.push(state_log.to_string_lossy().to_string());
            let status = Command::new("tail")
                .args(&args)
                .status()
                .context("failed to run tail on log file")?;
            if status.success() {
                return Ok(());
            }
            std::process::exit(status.code().unwrap_or(1));
        }
        let content = std::fs::read_to_string(&state_log).context("failed to read log file")?;
        let log_lines: Vec<&str> = content.lines().collect();
        let start = log_lines.len().saturating_sub(lines as usize);
        for line in &log_lines[start..] {
            println!("{line}");
        }
        return Ok(());
    }

    bail!(
        "No log file found at {}. Start the daemon or check journalctl.",
        state_log.display()
    );
}
