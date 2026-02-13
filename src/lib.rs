//! Collections of objects with typed indices and buildin identifier support.

#![deny(missing_docs)]
// `derivative` does not implement `Clone` the correct way if it also implements `Copy`
#![allow(clippy::non_canonical_clone_impl)]

mod collection;
mod error;
mod macros;

pub use crate::collection::*;
pub use crate::error::*;
