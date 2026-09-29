//! Package diagnostic checks for doctor.

pub mod check;
pub mod fmt;
pub mod query;

#[cfg(test)]
mod tests;

pub use check::*;
