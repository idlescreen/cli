// SPDX-License-Identifier: MIT

//! Environment and protocol soft-checks for doctor.

use super::checks::{CheckResult, fail, ok, warn};

pub fn check_wayland() -> CheckResult {
    match std::env::var("WAYLAND_DISPLAY") {
        Ok(val) if !val.is_empty() => ok("Environment", format!("WAYLAND_DISPLAY={val}")),
        _ => fail(
            "Environment",
            "WAYLAND_DISPLAY missing; IdleScreen needs a Wayland session",
        )
        .with_fix("run inside a Wayland session (not ssh/VT)"),
    }
}

/// Soft protocol/DE hints — an unrecognized DE warns rather than fails.
pub fn check_protocol_hints() -> CheckResult {
    let de = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    if std::env::var("WAYLAND_DISPLAY").is_err() {
        return fail(
            "Protocols",
            "WAYLAND_DISPLAY unset; need ext-idle-notify-v1 and zwlr_layer_shell_v1",
        )
        .with_fix("run inside a Wayland session");
    }
    let known = [
        "cosmic", "hyprland", "sway", "niri", "river", "wayfire", "kde", "plasma",
    ];
    let lower = de.to_ascii_lowercase();
    let is_gnome = lower.contains("gnome");
    let friendly = known.iter().any(|k| lower.contains(k));
    let de_label = if de.is_empty() {
        "unknown"
    } else {
        de.as_str()
    };
    if is_gnome {
        ok(
            "Protocols",
            format!("WAYLAND_DISPLAY set; DE='{de_label}' (GNOME Mutter: Mutter IdleMonitor + session-lock/xdg-shell)"),
        )
    } else if friendly || de.is_empty() {
        ok(
            "Protocols",
            format!("WAYLAND_DISPLAY set; DE='{de_label}' (need idle-notify + layer-shell)"),
        )
    } else {
        warn(
            "Protocols",
            format!("DE='{de_label}' unrecognized — may lack layer-shell/idle-notify"),
        )
        .with_fix("see docs/BOUNDARIES.md for the compositor support matrix")
    }
}

pub fn check_upower() -> CheckResult {
    let conn = match zbus::blocking::Connection::system() {
        Ok(c) => c,
        Err(_) => {
            return warn(
                "UPower Service",
                "cannot connect to system bus — battery status unavailable",
            )
            .with_fix("ensure dbus system broker is running");
        }
    };

    let proxy = match zbus::blocking::Proxy::new(
        &conn,
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower",
        "org.freedesktop.UPower",
    ) {
        Ok(p) => p,
        Err(e) => {
            return warn("UPower Service", format!("UPower proxy error: {e}"));
        }
    };

    match proxy.get_property::<bool>("OnBattery") {
        Ok(on_battery) => ok(
            "UPower Service",
            format!("available (on_battery={on_battery})"),
        ),
        Err(e) => warn("UPower Service", format!("UPower unavailable: {e}"))
            .with_fix("install upower or start upower.service"),
    }
}

pub fn check_audio_monitor() -> CheckResult {
    let rt_dir = match std::env::var("XDG_RUNTIME_DIR") {
        Ok(dir) => std::path::PathBuf::from(dir),
        Err(_) => {
            return warn(
                "Audio Monitor",
                "XDG_RUNTIME_DIR unset — audio monitor streams unavailable",
            )
            .with_fix("run inside a desktop user session");
        }
    };

    let has_pulse = rt_dir.join("pulse/native").exists();
    let has_pw = rt_dir.join("pipewire-0").exists();

    if has_pw {
        ok("Audio Monitor", "PipeWire monitor stream available")
    } else if has_pulse {
        ok("Audio Monitor", "PulseAudio monitor stream available")
    } else {
        warn(
            "Audio Monitor",
            "neither PipeWire nor PulseAudio socket found — audio-reactive savers will run silent",
        )
        .with_fix("ensure pipewire or pulseaudio is running in user session")
    }
}

pub fn check_portal_settings() -> CheckResult {
    let conn = match zbus::blocking::Connection::session() {
        Ok(c) => c,
        Err(_) => {
            return warn(
                "Portal Settings",
                "cannot connect to session bus — theme detection unavailable",
            )
            .with_fix("run inside a graphical user session");
        }
    };

    let proxy = match zbus::blocking::Proxy::new(
        &conn,
        "org.freedesktop.portal.Desktop",
        "/org/freedesktop/portal/desktop",
        "org.freedesktop.portal.Settings",
    ) {
        Ok(p) => p,
        Err(e) => {
            return warn("Portal Settings", format!("portal proxy error: {e}"));
        }
    };

    let reply: std::result::Result<(zbus::zvariant::OwnedValue,), _> =
        proxy.call("Read", &("org.freedesktop.appearance", "color-scheme"));
    match reply {
        Ok(_) => ok("Portal Settings", "desktop appearance portal available"),
        Err(e) => warn(
            "Portal Settings",
            format!("portal appearance unavailable ({e})"),
        )
        .with_fix("install xdg-desktop-portal and a DE portal backend"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_wayland() {
        let res = check_wayland();
        assert_eq!(res.name, "Environment");
    }

    #[test]
    fn test_check_protocol_hints() {
        let res = check_protocol_hints();
        assert_eq!(res.name, "Protocols");
    }

    #[test]
    fn test_check_protocol_hints_gnome() {
        let prev_de = std::env::var("XDG_CURRENT_DESKTOP").ok();
        let prev_wl = std::env::var("WAYLAND_DISPLAY").ok();
        unsafe {
            std::env::set_var("WAYLAND_DISPLAY", "wayland-0");
            std::env::set_var("XDG_CURRENT_DESKTOP", "GNOME");
        }
        let res = check_protocol_hints();
        assert_eq!(res.name, "Protocols");
        assert!(res.passed());
        assert!(res.detail.contains("GNOME Mutter"));
        if let Some(de) = prev_de {
            unsafe { std::env::set_var("XDG_CURRENT_DESKTOP", de); }
        } else {
            unsafe { std::env::remove_var("XDG_CURRENT_DESKTOP"); }
        }
        if let Some(wl) = prev_wl {
            unsafe { std::env::set_var("WAYLAND_DISPLAY", wl); }
        } else {
            unsafe { std::env::remove_var("WAYLAND_DISPLAY"); }
        }
    }

    #[test]
    fn test_check_upower() {
        let res = check_upower();
        assert_eq!(res.name, "UPower Service");
        assert!(res.passed());
    }

    #[test]
    fn test_check_audio_monitor() {
        let res = check_audio_monitor();
        assert_eq!(res.name, "Audio Monitor");
        assert!(res.passed());
    }

    #[test]
    fn test_check_portal_settings() {
        let res = check_portal_settings();
        assert_eq!(res.name, "Portal Settings");
        assert!(res.passed());
    }
}
