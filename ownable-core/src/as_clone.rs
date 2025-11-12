use crate::as_impl::impl_as;
use crate::traits::{IntoOwned, ToBorrowed, ToOwned};
use alloc::string::String;
use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};

/// Transparent wrapper for [`Clone`]able types to support all traits (by cloning).
///
/// ```rust
/// # use ownable_core::{AsClone, IntoOwned};
/// #[derive(Clone)]
/// struct NotIntoOwned;
///
/// fn requires_into_owned<O: IntoOwned>(o: O) {
///   // do stuff
/// }
///
/// // `AsClone<NotIntoOwned>` implements `IntoOwned` through `Clone`
/// requires_into_owned(AsClone(NotIntoOwned));
/// ```
///
/// All trait impls work on the inner value as if there is no layer in between
/// (e.g. `Display` does not add a `AsClone` to the output).
#[repr(transparent)]
pub struct AsClone<T: Clone>(pub T);

impl<T: Clone> ToBorrowed<'_> for AsClone<T> {
    #[inline(always)]
    fn to_borrowed(&self) -> Self {
        AsClone(self.0.clone())
    }
}

impl<T: Clone> ToOwned for AsClone<T> {
    type Owned = AsClone<T>;

    #[inline(always)]
    fn to_owned(&self) -> Self::Owned {
        AsClone(self.0.clone())
    }
}

impl<T: Clone> IntoOwned for AsClone<T> {
    type Owned = AsClone<T>;

    #[inline(always)]
    fn into_owned(self) -> Self::Owned {
        AsClone(self.0)
    }
}

impl_as!(AsClone, Clone);

// Clone specific

impl From<&str> for AsClone<String> {
    #[inline]
    fn from(value: &str) -> Self {
        AsClone(value.into())
    }
}
