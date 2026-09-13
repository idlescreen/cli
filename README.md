# cli

Daemon protocol commands — `status`, `preview`, `config`, `saver`,
`inhibitors`, `doctor`, `self-update` — over D-Bus. Part of
[IdleScreen](https://idlescreen.github.io) — modular Wayland
screensavers for Linux.

The installed binary is **`idle-cli`**. The
[`idlescreen`](https://github.com/idlescreen/idlescreen) router owns
`/usr/bin/idlescreen` and forwards every daemon verb here.

## Install

Ships with the `idlescreen` product package. On its own:

```sh
idlescreen install cli
```

## Commands

```sh
idle-cli status             # or: idlescreen status
idle-cli preview aurora     # or: idlescreen preview aurora
idle-cli saver set beams    # or: idlescreen saver set beams
idle-cli doctor             # or: idlescreen doctor
```

## License

Apache-2.0 · © 2026 IdleScreen
