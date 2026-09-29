// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Ship the static shell completions into `target/<profile>/completions/`
//! so the deb/rpm packaging assets can install them.

use std::fs;
use std::path::Path;

fn fail(msg: &str) -> ! {
    eprintln!("idle-cli build.rs: {msg}");
    std::process::exit(1)
}

fn main() {
    // OUT_DIR is target/<profile>/build/<pkg>-<hash>/out — walk up to the
    // profile dir so assets land at target/release/completions/.
    let dir = std::env::var_os("OUT_DIR")
        .and_then(|out| {
            Path::new(&out)
                .ancestors()
                .nth(3)
                .map(|p| p.join("completions"))
        })
        .unwrap_or_else(|| fail("unexpected OUT_DIR layout"));
    fs::create_dir_all(&dir).unwrap_or_else(|e| fail(&format!("create {}: {e}", dir.display())));

    for file in [
        "idlescreen",
        "_idlescreen",
        "idlescreen.fish",
        "idlescreen.elv",
        "_idlescreen.ps1",
    ] {
        let src = Path::new("src/completions").join(file);
        println!("cargo:rerun-if-changed={}", src.display());
        fs::copy(&src, dir.join(file))
            .unwrap_or_else(|e| fail(&format!("copy {}: {e}", src.display())));
    }
}
