// SPDX-License-Identifier: MIT

//! Session-shell integration diagnostics.

pub mod classify;
pub mod facts;

#[cfg(test)]
mod tests;

use crate::doctor::checks::CheckResult;

/// Report whether a session shell's idle trigger is correctly handing off to
/// IdleScreen. See [`classify`] for the rules.
pub fn check_session_shell_integration() -> CheckResult {
    classify::classify(&facts::gather())
}
