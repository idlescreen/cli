// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Help text for top-level and per-subcommand topics.

// ---------------------------------------------------------------------------
// Help text
// ---------------------------------------------------------------------------

pub(crate) fn help_top() -> String {
    "Control the IdleScreen Wayland screensaver daemon.\n\
     Run `idlescreen <command> --help` for subcommand details.\n\
     \n\
     Usage: idlescreen [OPTIONS] <COMMAND>\n\
     \n\
     Commands:\n\
     \x20 status        Show daemon state (running, idle, saver, inhibitors) [alias: st]\n\
     \x20 config        View or change daemon configuration [alias: cfg]\n\
     \x20 enable        Turn the idle screensaver on [alias: on]\n\
     \x20 disable       Turn the idle screensaver off [alias: off]\n\
     \x20 timeout       Set or show the idle timeout in minutes (1-240) [alias: t]\n\
     \x20 saver         Show or change the active saver [alias: sv]\n\
     \x20 list          List installed savers [alias: ls]\n\
     \x20 inhibitors    List active idle inhibitors [alias: inhib]\n\
     \x20 inhibit       Run a command while holding an idle inhibitor [alias: hold]\n\
     \x20 start         Activate the screensaver now [alias: activate]\n\
     \x20 preview       Preview a saver fullscreen [alias: p]\n\
     \x20 stop          Stop the running preview or idle presentation [alias: x]\n\
     \x20 fps-overlay   FPS overlay: on, off, or status [alias: fps]\n\
     \x20 render-scale  Render scale: 0.25-1.0, 'default', or status [alias: scale]\n\
     \x20 interactive   Interactive console panel [alias: i]\n\
     \x20 restart       Restart the idle-daemon user service [alias: rs]\n\
     \x20 logs          Show daemon logs [alias: log]\n\
     \x20 doctor        Run diagnostics [alias: doc]\n\
     \x20 clean         Remove stale run state and log caches [alias: cl]\n\
     \x20 completion    Print a shell completion script to stdout [alias: comp]\n\
     \x20 bug-report    Print a sanitized diagnostics bundle [alias: bug]\n\
     \x20 self-update   Check for updates and upgrade packages [aliases: update, upgrade]\n\
     \x20 tui           Launch the full-screen TUI [alias: ui]\n\
     \x20 version       Print CLI version [alias: v]\n\
     \x20 about         Print version plus project info [alias: info]\n\
     \x20 help          Print this message or the help of the given subcommand(s)\n\
     \n\
     Options:\n\
     \x20 -q, --quiet     Suppress confirmations and other non-essential output\n\
     \x20 -h, --help      Print help\n\
     \x20 -V, --version   Print version"
        .to_string()
}

pub(crate) fn help_for(name: &str) -> String {
    let (about, usage, opts) = match name {
        "status" => (
            "Show daemon state (running, idle, saver, inhibitors)",
            "idlescreen status [OPTIONS]",
            "  -j, --json   Machine-readable JSON output",
        ),
        "config" => (
            "View or change daemon configuration",
            "idlescreen config [OPTIONS] [COMMAND]",
            "  -j, --json   Machine-readable JSON output\n\n\
             Commands:\n  list   Show all configuration keys\n  \
             get    Show one configuration value: get <key>\n  \
             set    Set a configuration value: set <key> <value>\n  \
             path   Print the configuration file path\n  \
             edit   Open the configuration file in $EDITOR\n  \
             reset  Restore defaults: reset [-y|--yes]",
        ),
        "saver" => (
            "Show or change the active saver",
            "idlescreen saver [OPTIONS] [COMMAND]",
            "  -j, --json   Machine-readable JSON output\n\n\
             Commands:\n  set <name>  Set the active saver ('random'/'none' for rotation)\n  \
             list        List installed savers",
        ),
        "timeout" => (
            "Set or show the idle timeout in minutes (1-240)",
            "idlescreen timeout [OPTIONS] [minutes]",
            "  -j, --json   Machine-readable JSON output",
        ),
        "list" | "inhibitors" => (
            "List installed savers / active idle inhibitors",
            "idlescreen list [OPTIONS]",
            "  -j, --json   Machine-readable JSON output",
        ),
        "inhibit" => (
            "Run a command while holding an idle inhibitor",
            "idlescreen inhibit [OPTIONS] <command>...",
            "  -r, --reason <reason>  Reason recorded for the inhibitor",
        ),
        "preview" => (
            "Preview a saver fullscreen",
            "idlescreen preview [OPTIONS] <name>",
            "  -t, --timeout <seconds>  Auto-stop after N seconds",
        ),
        "fps-overlay" => (
            "FPS overlay: on, off, or status",
            "idlescreen fps-overlay [on|off|status]",
            "",
        ),
        "render-scale" => (
            "Render scale: 0.25-1.0, 'default', or status",
            "idlescreen render-scale [value]",
            "",
        ),
        "logs" => (
            "Show daemon logs (journalctl --user -u idle-daemon)",
            "idlescreen logs [OPTIONS]",
            "  -f, --follow       Follow new entries (Ctrl-C to stop)\n  \
             -n, --lines <n>    Number of recent entries to show [default: 100]",
        ),
        "doctor" => (
            "Run diagnostics",
            "idlescreen doctor [OPTIONS]",
            "  -f, --fix    Attempt to repair common issues\n  \
             -j, --json   Machine-readable JSON output",
        ),
        "clean" => (
            "Remove stale run state and log caches",
            "idlescreen clean [OPTIONS]",
            "  -n, --dry-run  Show what would be removed without deleting anything",
        ),
        "completion" => (
            "Print a shell completion script to stdout",
            "idlescreen completion <shell>",
            "  shell: bash, zsh, fish, nushell (nu), powershell (pwsh), elvish",
        ),
        "self-update" => (
            "Check for updates and upgrade installed IdleScreen packages",
            "idlescreen self-update [OPTIONS]",
            "  -c, --check  Report update status without upgrading anything",
        ),
        "tui" => (
            "Launch the full-screen TUI",
            "idlescreen tui [args]...",
            "  args are forwarded to idle-tui",
        ),
        "version" => (
            "Print CLI version",
            "idlescreen version [OPTIONS]",
            "  -l, --long   Extended info (same as `about`)\n  \
             -j, --json   Machine-readable JSON output",
        ),
        _ => ("Control the IdleScreen daemon", "idlescreen <COMMAND>", ""),
    };
    let mut s = format!("{about}\n\nUsage: {usage}\n\nOptions:");
    if !opts.is_empty() {
        s.push('\n');
        s.push_str(opts);
    }
    s.push_str("\n  -q, --quiet   Suppress confirmations and other non-essential output\n  -h, --help    Print help");
    s
}
