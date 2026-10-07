// SPDX-License-Identifier: MIT

//! Pure judgement for the session-shell integration check.
//!
//! The integration is two independent things — a `PATH` shadow and a config
//! flag — that can each break while the other looks fine. Neither failure is
//! visible from the daemon's own status, which is the whole reason this check
//! exists.

use super::facts::IntegrationFacts;
use crate::doctor::checks::{CheckResult, Severity};

const NAME: &str = "Session shell integration";

pub fn classify(facts: &IntegrationFacts) -> CheckResult {
    // Reported for every state that would otherwise present: a live inhibitor
    // silently suppresses the screensaver and looks like a broken install.
    let inhibited = match facts.inhibitor_count {
        Some(n) if n > 0 => Some(n),
        _ => None,
    };

    let base = classify_integration(facts);

    if let Some(count) = inhibited
        && base.severity == Severity::Ok
    {
        return CheckResult {
            name: NAME,
            severity: Severity::Warn,
            detail: format!(
                "{} — but {count} idle inhibitor(s) are active, which suppresses the \
                 screensaver",
                base.detail
            ),
            fix: Some("idlescreen inhibitors".to_string()),
        };
    }
    base
}

fn classify_integration(facts: &IntegrationFacts) -> CheckResult {
    if !facts.shell_present && !facts.shim_present {
        return ok("no session shell that owns idle — IdleScreen drives its own timer");
    }

    if facts.shim_present {
        if !facts.router_present {
            return fail(
                "integration shim is installed but `idlescreen` is not on PATH — the \
                 screensaver will never start",
            );
        }
        if facts.idle_enabled == Some(true) {
            return warn_with_fix(
                "integration shim is installed and idle_enabled is true — the shell and \
                 IdleScreen will both present, at different delays",
                "idlescreen disable",
            );
        }
        return ok("shell owns idle timing; IdleScreen draws and the shim is wired");
    }

    // No shim. If IdleScreen's own timer is off, nothing will ever present.
    if facts.shell_present && facts.idle_enabled == Some(false) {
        return warn_with_fix(
            "IdleScreen's own idle timer is off but no integration shim is installed — \
             nothing will present when the session idles",
            "idlescreen enable",
        );
    }

    ok("integration not applied; IdleScreen drives its own timer")
}

fn ok(detail: &str) -> CheckResult {
    CheckResult {
        name: NAME,
        severity: Severity::Ok,
        detail: detail.to_string(),
        fix: None,
    }
}

fn warn_with_fix(detail: &str, fix: &str) -> CheckResult {
    CheckResult {
        name: NAME,
        severity: Severity::Warn,
        detail: detail.to_string(),
        fix: Some(fix.to_string()),
    }
}

fn fail(detail: &str) -> CheckResult {
    CheckResult {
        name: NAME,
        severity: Severity::Fail,
        detail: detail.to_string(),
        fix: Some("reinstall idlescreen".to_string()),
    }
}
