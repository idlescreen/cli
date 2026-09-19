// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! Minimal error plumbing replacing `anyhow`: a boxed error with context
//! chaining, `Result`, `Context`, `bail!`, `anyhow!`, `downcast_ref`, and
//! anyhow-style `{:#}` (single-line "ctx: cause: root") display.

use std::error::Error as StdError;
use std::fmt;

/// Boxed error chain. `context()` wraps the current error with a message.
pub struct Error {
    inner: Box<dyn StdError + Send + Sync>,
}

/// Leaf error carrying a plain message.
struct Msg(String);

impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl fmt::Debug for Msg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl StdError for Msg {}

/// Context layer: prints as `msg` and exposes the wrapped error as source.
struct Ctx {
    msg: String,
    source: Box<dyn StdError + Send + Sync>,
}

impl fmt::Display for Ctx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)
    }
}
impl fmt::Debug for Ctx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.msg, self.source)
    }
}
impl StdError for Ctx {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.source)
    }
}

impl Error {
    /// Error from a plain message (no source).
    pub fn msg(s: impl Into<String>) -> Self {
        Self {
            inner: Box::new(Msg(s.into())),
        }
    }

    /// First `T` in the chain, if any (matches `anyhow::Error::downcast_ref`).
    pub fn downcast_ref<T: StdError + 'static>(&self) -> Option<&T> {
        let mut cur: &(dyn StdError + 'static) = &*self.inner;
        loop {
            if let Some(t) = cur.downcast_ref::<T>() {
                return Some(t);
            }
            cur = cur.source()?;
        }
    }
}

impl<E> From<E> for Error
where
    E: StdError + Send + Sync + 'static,
{
    fn from(e: E) -> Self {
        Self { inner: Box::new(e) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // anyhow `{:#}`: "context: cause: root" on one line.
            let mut first = true;
            let mut cur: &(dyn StdError + 'static) = &*self.inner;
            loop {
                if !first {
                    write!(f, ": ")?;
                }
                first = false;
                write!(f, "{cur}")?;
                match cur.source() {
                    Some(s) => cur = s,
                    None => return Ok(()),
                }
            }
        }
        write!(f, "{}", self.inner)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// `anyhow::Result<T>` equivalent.
pub type Result<T> = std::result::Result<T, Error>;

/// Attach context to a `Result` or `Option` (anyhow's `Context` trait).
pub trait Context<T> {
    fn context(self, msg: impl Into<String>) -> Result<T>;
    fn with_context(self, f: impl FnOnce() -> String) -> Result<T>;
}

impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error {
            inner: Box::new(Ctx {
                msg: msg.into(),
                source: Box::new(e),
            }),
        })
    }

    fn with_context(self, f: impl FnOnce() -> String) -> Result<T> {
        self.map_err(|e| Error {
            inner: Box::new(Ctx {
                msg: f(),
                source: Box::new(e),
            }),
        })
    }
}

impl<T> Context<T> for std::result::Result<T, Error> {
    fn context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error {
            inner: Box::new(Ctx {
                msg: msg.into(),
                source: e.inner,
            }),
        })
    }

    fn with_context(self, f: impl FnOnce() -> String) -> Result<T> {
        self.map_err(|e| Error {
            inner: Box::new(Ctx {
                msg: f(),
                source: e.inner,
            }),
        })
    }
}

impl<T> Context<T> for Option<T> {
    fn context(self, msg: impl Into<String>) -> Result<T> {
        self.ok_or_else(|| Error::msg(msg.into()))
    }

    fn with_context(self, f: impl FnOnce() -> String) -> Result<T> {
        self.ok_or_else(|| Error::msg(f()))
    }
}

macro_rules! anyhow {
    ($($arg:tt)*) => {
        $crate::err::Error::msg(format!($($arg)*))
    };
}
pub(crate) use anyhow;

macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::err::anyhow!($($arg)*))
    };
}
pub(crate) use bail;

#[cfg(test)]
mod tests {
    use super::*;

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
