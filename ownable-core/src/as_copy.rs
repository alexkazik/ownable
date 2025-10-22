use crate::as_impl::impl_as;
use crate::traits::{IntoOwned, ToBorrowed, ToOwned};
use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};

/// Transparent wrapper for [`Copy`]able types to support all traits (by copying).
///
/// ```rust
/// # use ownable_core::{AsCopy, IntoOwned};
/// #[derive(Clone, Copy)]
/// struct NotIntoOwned;
///
/// fn requires_to_owned<O: IntoOwned>(o: O) {
///   // do stuff
/// }
///
/// // `AsCopy<NotIntoOwned>` implements `IntoOwned` through `Copy`
/// requires_to_owned(AsCopy(NotIntoOwned));
/// ```
///
/// All trait impls work on the inner value as if there is no layer in between
/// (e.g. `Display` does not add a `AsCopy` to the output).
#[repr(transparent)]
pub struct AsCopy<T: Copy>(pub T);

impl<T: Copy> ToBorrowed<'_> for AsCopy<T> {
    #[inline(always)]
    fn to_borrowed(&self) -> Self {
        AsCopy(self.0)
    }
}

impl<T: Copy> ToOwned for AsCopy<T> {
    type Owned = AsCopy<T>;

    #[inline(always)]
    fn to_owned(&self) -> Self::Owned {
        AsCopy(self.0)
    }
}

impl<T: Copy> IntoOwned for AsCopy<T> {
    type Owned = AsCopy<T>;

    #[inline(always)]
    fn into_owned(self) -> Self::Owned {
        AsCopy(self.0)
    }
}

impl_as!(AsCopy, Copy);
