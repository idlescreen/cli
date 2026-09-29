// SPDX-License-Identifier: MIT

#![cfg(test)]

//! idle-err shape tests: anyhow-compatible surface.

#[cfg(test)]
mod tests {
    use crate::err::*;

    #[test]
    fn msg_displays_text() {
        let e = Error::msg("boom");
        assert_eq!(format!("{e}"), "boom");
        assert_eq!(format!("{e:#}"), "boom");
    }

    #[test]
    fn context_chains_like_anyhow() {
        let leaf = std::io::Error::new(std::io::ErrorKind::NotFound, "no file");
        let e = std::result::Result::<(), _>::Err(leaf)
            .context("reading config")
            .context("loading settings")
            .unwrap_err();
        assert_eq!(format!("{e}"), "loading settings");
        assert_eq!(
            format!("{e:#}"),
            "loading settings: reading config: no file"
        );
    }

    #[test]
    fn downcast_finds_source_in_chain() {
        let leaf = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let e = std::result::Result::<(), _>::Err(leaf)
            .context("outer")
            .context("outermost")
            .unwrap_err();
        let io_err = e
            .downcast_ref::<std::io::Error>()
            .expect("io::Error in chain");
        assert_eq!(io_err.kind(), std::io::ErrorKind::PermissionDenied);
        assert!(e.downcast_ref::<std::fmt::Error>().is_none());
    }

    #[test]
    fn with_context_is_lazy() {
        let ok: std::result::Result<u32, std::io::Error> = Ok(7);
        let mut called = false;
        let v = ok
            .with_context(|| {
                called = true;
                "should not run".to_string()
            })
            .unwrap();
        assert_eq!(v, 7);
        assert!(!called, "context closure must not run on Ok");
    }

    #[test]
    fn context_trait_on_option() {
        let none: Option<u32> = None;
        assert_eq!(
            format!("{}", none.context("was empty").unwrap_err()),
            "was empty"
        );
        assert_eq!(Some(3u32).context("unused").unwrap(), 3);
    }

    #[test]
    fn context_trait_on_own_result_rewraps() {
        let r: Result<()> = Err(Error::msg("base"));
        let e = r.context("wrapped").unwrap_err();
        assert_eq!(format!("{e:#}"), "wrapped: base");
    }

    #[test]
    fn anyhow_macro_formats() {
        let n = 41;
        let e = anyhow!("value {n} is off by one");
        assert_eq!(format!("{e}"), "value 41 is off by one");
    }

    #[test]
    fn bail_early_returns() {
        fn f() -> Result<()> {
            bail!("stop {0}", 3);
        }
        assert_eq!(format!("{}", f().unwrap_err()), "stop 3");
    }

    #[test]
    fn from_any_std_error() {
        let e: Error = std::io::Error::new(std::io::ErrorKind::TimedOut, "slow").into();
        assert_eq!(format!("{e}"), "slow");
    }
}
