# idle-cli

Protocol commands for the IdleScreen daemon — `preview`, `stop`, `list`,
`status`, `config`, `doctor`, `self-update`, and friends over D-Bus.

The installed binary is **`idle-cli`**. The [`idlescreen`
router](https://github.com/idlescreen/idlescreen) owns `/usr/bin/idlescreen`
and forwards every daemon verb here, so `idlescreen <verb>` keeps working —
this package depends on it.

## Usage

```sh
idle-cli status          # or: idlescreen status
idle-cli preview aurora  # or: idlescreen preview aurora
idle-cli doctor
```

## Development

Path dependency: a sibling [`runtime`](https://github.com/idlescreen/runtime)
checkout (or a `runtime` symlink) provides `idle-dbus`:

```sh
git clone https://github.com/idlescreen/runtime ../runtime   # if needed
cargo build
```

## License

Apache-2.0
