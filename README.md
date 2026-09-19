# cli

[![studio2201 gate](https://github.com/idlescreen/cli/actions/workflows/studio2201.yml/badge.svg)](https://github.com/idlescreen/cli/actions/workflows/studio2201.yml)

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
