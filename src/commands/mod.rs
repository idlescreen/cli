// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! CLI command handlers. Public surface is re-exported for `main`
//! stability.
//!
//! Layout: one submodule per command group, each holding the
//! `cmd_*` entry points consumed by the clap dispatcher:
//!
//!   - [`control`] — runtime-mutating commands (preview, render
//!     scale, saver set/show, timeout, inhibitors, fps overlay,
//!     inhibit, list).
//!   - [`status`]  — read-only commands (status, version).
//!   - [`inhibitors_fmt`] — pretty-printing helpers shared by the
//!     `cmd_inhibitors` path and the status page.
//!
//! Per RULES.md §2, each `cmd_*` entry point lives in the file
//! that owns its command group — `commands/mod.rs` is the
//! re-export hub, not a place for inline implementations. New
//! commands land in `control.rs` (or a new sibling module
//! matching the command's domain), not here.

mod control;
mod inhibitors_fmt;
mod status;

pub use control::{
    cmd_fps_overlay, cmd_inhibit, cmd_inhibitors, cmd_list, cmd_preview, cmd_render_scale,
    cmd_saver_set, cmd_saver_show, cmd_timeout,
};
pub use status::{cmd_status, print_version};