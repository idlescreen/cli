# cli

[![snip](https://img.shields.io/github/actions/workflow/status/idlescreen/cli/snip.yml?label=snip&logo=shield)](https://github.com/idlescreen/cli/actions/workflows/snip.yml)
[![vigil](https://img.shields.io/github/actions/workflow/status/idlescreen/cli/vigil.yml?label=vigil&logo=shield)](https://github.com/idlescreen/cli/actions/workflows/vigil.yml)
[![aegis](https://img.shields.io/github/actions/workflow/status/idlescreen/cli/aegis.yml?label=aegis&logo=shield)](https://github.com/idlescreen/cli/actions/workflows/aegis.yml)
[![proven](https://img.shields.io/github/actions/workflow/status/idlescreen/cli/proven.yml?label=proven&logo=shield)](https://github.com/idlescreen/cli/actions/workflows/proven.yml)
[![boneyard](https://img.shields.io/github/actions/workflow/status/idlescreen/cli/boneyard.yml?label=boneyard&logo=shield)](https://github.com/idlescreen/cli/actions/workflows/boneyard.yml)

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
