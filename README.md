# cli

[![studio2201 Suite](https://img.shields.io/badge/studio2201-5%2F5%20Verified-2f6f5e?logo=shield)](https://studio2201.com/agents#badges)

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
idle-cli doctor --fix    # or: idlescreen doctor --fix
idle-cli preview storm   # or: idlescreen preview storm
idle-cli self-update     # or: idlescreen update
idle-cli tui             # or: idlescreen tui
```

## License

Apache-2.0 · © 2026 IdleScreen

---

<div align="center">

[![Necrometer](necrometer.svg)](https://necrometer.dev/?u=idlescreen)

</div>
