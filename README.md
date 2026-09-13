# cli

Daemon protocol commands — `status`, `preview`, `config`, `saver`,
`inhibitors`, `doctor`, `self-update` — over D-Bus.
Part of [IdleScreen](https://idlescreen.github.io) — modular Wayland
screensavers for Linux.

The installed binary is **`idle-cli`**. The
[`idlescreen`](https://github.com/idlescreen/idlescreen) router owns
`/usr/bin/idlescreen` and forwards every daemon verb here, so
`idlescreen <verb>` keeps working.

## Use

```sh
idle-cli status           # or: idlescreen status
idle-cli preview aurora   # or: idlescreen preview aurora
idle-cli doctor           # or: idlescreen doctor
```

## Develop

Path dependency: a `runtime/` checkout inside this repo (or a symlink to a
sibling clone) provides `idle-dbus`.

```sh
git clone https://github.com/idlescreen/cli.git && cd cli
git clone https://github.com/idlescreen/runtime runtime    # path dep
cargo build && cargo test
```

## License

Apache-2.0 · © 2026 IdleScreen
