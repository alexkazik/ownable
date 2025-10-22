#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]

//! Please see the [ownable](https://docs.rs/ownable) crate for a more comprehensive overview.
//!
//! This crate contains the core types and traits used by `ownable` available without all of the
//! macro related dependencies that get used for the derive macros. If you don't rely on the derive
//! macros then you may benefit from pulling in fewer dependencies by relying directly on
//! `ownable-core`.

extern crate alloc;

pub use crate::as_clone::AsClone;
pub use crate::as_copy::AsCopy;
pub use crate::traits::{IntoOwned, ToBorrowed, ToOwned};

mod as_clone;
mod as_copy;
mod as_impl;
mod traits;
