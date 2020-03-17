//! Collections of objects with typed indices and buildin identifier support.

#![deny(missing_docs)]

/// The error type used by the crate.
pub type Error = failure::Error;

/// The corresponding result type used by the crate.
pub type Result<T> = std::result::Result<T, Error>;

mod collection;

pub use crate::collection::*;
