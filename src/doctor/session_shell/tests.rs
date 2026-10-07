// SPDX-License-Identifier: MIT

use super::classify::classify;
use super::facts::IntegrationFacts;
use crate::doctor::checks::Severity;

fn facts() -> IntegrationFacts {
    IntegrationFacts {
        shell_present: true,
        shim_present: true,
        router_present: true,
        idle_enabled: Some(false),
        inhibitor_count: Some(0),
    }
}

#[test]
fn healthy_integration_passes() {
    let result = classify(&facts());
    assert_eq!(result.severity, Severity::Ok, "{}", result.detail);
}

#[test]
fn shim_without_the_router_fails() {
    // The shim is a dead end without the binary it execs: nothing presents
    // and no error explains why.
    let f = facts();
    let result = classify(&IntegrationFacts {
        router_present: false,
        ..f
    });
    assert_eq!(result.severity, Severity::Fail);
    assert!(result.detail.contains("not on PATH"), "{}", result.detail);
}

#[test]
fn shim_with_own_timer_live_warns() {
    // Both timers running is the double-fire case: two savers, two delays.
    let result = classify(&IntegrationFacts {
        idle_enabled: Some(true),
        ..facts()
    });
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.fix.as_deref(), Some("idlescreen disable"));
}

#[test]
fn timer_off_without_a_shim_warns() {
    // Half-applied: the shim was removed or never installed, and IdleScreen
    // will never present on its own either.
    let result = classify(&IntegrationFacts {
        shim_present: false,
        ..facts()
    });
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.fix.as_deref(), Some("idlescreen enable"));
}

#[test]
fn no_shell_and_no_shim_is_healthy() {
    let result = classify(&IntegrationFacts {
        shell_present: false,
        shim_present: false,
        router_present: true,
        idle_enabled: Some(true),
        inhibitor_count: Some(0),
    });
    assert_eq!(result.severity, Severity::Ok);
}

#[test]
fn shim_without_a_shell_still_checks_the_router() {
    // A leftover shim on a host that no longer runs the shell is a stale
    // integration, not a healthy one.
    let result = classify(&IntegrationFacts {
        shell_present: false,
        router_present: false,
        ..facts()
    });
    assert_eq!(result.severity, Severity::Fail);
}

#[test]
fn unset_idle_enabled_with_a_shim_is_accepted() {
    // The key defaults to true, but the daemon may not have written config
    // yet. Don't cry wolf on a first run.
    let result = classify(&IntegrationFacts {
        idle_enabled: None,
        ..facts()
    });
    assert_eq!(result.severity, Severity::Ok);
}

#[test]
fn live_inhibitors_warn_even_when_the_integration_is_healthy() {
    // The worst-looking bug report in this area is "the screensaver never
    // appears", and the usual cause is a video player holding an inhibitor.
    let result = classify(&IntegrationFacts {
        inhibitor_count: Some(2),
        ..facts()
    });
    assert_eq!(result.severity, Severity::Warn);
    assert!(
        result.detail.contains("2 idle inhibitor"),
        "{}",
        result.detail
    );
    assert_eq!(result.fix.as_deref(), Some("idlescreen inhibitors"));
}

#[test]
fn an_unreachable_daemon_is_not_reported_as_zero_inhibitors() {
    let result = classify(&IntegrationFacts {
        inhibitor_count: None,
        ..facts()
    });
    assert_eq!(result.severity, Severity::Ok);
}
