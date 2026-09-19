# cli

<div align="center">

| Security Pillar | Verification Badge |
| :--- | :---: |
| **Platform Standard** | [![secured by studio2201][b-studio]][u-home] |
| **Credential Defense** | [![snip][b-snip]][u-snip] |
| **Supply Chain Surface** | [![vigil][b-vigil]][u-vigil] |
| **Post-Quantum Cryptography** | [![aegis][b-aegis]][u-aegis] |
| **Build Provenance & SLSA** | [![proven][b-proven]][u-proven] |
| **Repository Governance** | [![boneyard][b-boneyard]][u-boneyard] |

[b-studio]: https://img.shields.io/badge/secured%20by-studio2201-2f6f5e?logo=shield
[u-home]: https://studio2201.com
[b-snip]: https://img.shields.io/badge/snip-0%20secrets-2f6f5e?logo=shield
[u-snip]: https://studio2201.com/snip
[b-vigil]: https://img.shields.io/badge/vigil-0%20dependencies-2f6f5e?logo=shield
[u-vigil]: https://studio2201.com/vigil
[b-aegis]: https://img.shields.io/badge/aegis-PQC%20compliant-2f6f5e?logo=shield
[u-aegis]: https://studio2201.com/aegis
[b-proven]: https://img.shields.io/badge/proven-ML--DSA--65%20verified-2f6f5e?logo=shield
[u-proven]: https://studio2201.com/proven
[b-boneyard]: https://img.shields.io/badge/boneyard-maintained-2f6f5e?logo=shield
[u-boneyard]: https://studio2201.com/boneyard

</div>

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
