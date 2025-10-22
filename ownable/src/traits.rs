//! These traits are the internals, which are used by the derive macros.
//!
//! There are intended to be always used qualified (in order to not conflict with the derived
//! functions and other traits/functions with an identical name).
//!
//! See [crate] for more information.
//!
//! If you don't rely on the derive macros then you may benefit from pulling in fewer dependencies
//! by relying directly on [ownable-core](https://docs.rs/ownable-core).

pub use ownable_core::{IntoOwned, ToBorrowed, ToOwned};
