//! These traits are the internals, which are used by the derive macros.
//!
//! There are intended to be always used qualified (in order to not conflict with the derived
//! functions and other traits/functions with an identical name).
//!
//! See [crate] for more information.

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use core::borrow::Borrow;

mod copy;
mod iter;
#[cfg(feature = "std")]
mod std;

/// Copy the structure and reference the original values.
///
/// This is always a deep copy of the structure.
pub trait ToBorrowed<'a> {
    /// Copy the structure and reference the original values.
    ///
    /// This is always a deep copy of the structure.
    #[must_use]
    fn to_borrowed(&'a self) -> Self;
}

/// Copy the structure and clone the original values.
///
/// This is always a deep copy.
pub trait ToOwned {
    /// The type after obtaining ownership, should be the same as before but with static lifetime.
    type Owned;
    /// Copy the structure and clone the original values.
    ///
    /// This is always a deep copy.
    #[must_use]
    fn to_owned(&self) -> Self::Owned;
}

/// Copy the structure and clone the original values if it's not owned.
///
/// This is always a deep copy of the structure.
pub trait IntoOwned {
    /// The type after obtaining ownership, should be the same as before but with static lifetime.
    type Owned;
    /// Copy the structure and clone the original values if it's not owned.
    ///
    /// This is always a deep copy of the structure.
    #[must_use]
    fn into_owned(self) -> Self::Owned;
}

// Cow

impl<'a, T: alloc::borrow::ToOwned + ?Sized> ToBorrowed<'a> for Cow<'a, T> {
    #[inline]
    fn to_borrowed(&'a self) -> Self {
        Cow::Borrowed(self.borrow())
    }
}

impl<T: alloc::borrow::ToOwned + ?Sized + 'static> ToOwned for Cow<'_, T> {
    type Owned = Cow<'static, T>;

    #[inline]
    fn to_owned(&self) -> Cow<'static, T> {
        Cow::Owned(T::to_owned(self.borrow()))
    }
}

impl<T: alloc::borrow::ToOwned + ?Sized + 'static> IntoOwned for Cow<'_, T> {
    type Owned = Cow<'static, T>;

    #[inline]
    fn into_owned(self) -> Cow<'static, T> {
        Cow::Owned(self.into_owned())
    }
}

// Option

impl<'a, T: ToBorrowed<'a>> ToBorrowed<'a> for Option<T> {
    #[inline]
    fn to_borrowed(&'a self) -> Self {
        self.as_ref().map(ToBorrowed::to_borrowed)
    }
}

impl<T: ToOwned> ToOwned for Option<T> {
    type Owned = Option<T::Owned>;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        self.as_ref().map(ToOwned::to_owned)
    }
}

impl<T: IntoOwned> IntoOwned for Option<T> {
    type Owned = Option<T::Owned>;

    #[inline]
    fn into_owned(self) -> Self::Owned {
        self.map(IntoOwned::into_owned)
    }
}

// Box<T>

impl<'a, T: ToBorrowed<'a>> ToBorrowed<'a> for Box<T> {
    #[inline]
    fn to_borrowed(&'a self) -> Self {
        Box::new(self.as_ref().to_borrowed())
    }
}

impl<T: ToOwned> ToOwned for Box<T> {
    type Owned = Box<T::Owned>;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        Box::new(self.as_ref().to_owned())
    }
}

impl<T: IntoOwned> IntoOwned for Box<T> {
    type Owned = Box<T::Owned>;

    #[inline]
    fn into_owned(self) -> Self::Owned {
        Box::new((*self).into_owned())
    }
}

// Box<[T]>

impl<'a, T: ToBorrowed<'a>> ToBorrowed<'a> for Box<[T]> {
    #[inline]
    fn to_borrowed(&'a self) -> Self {
        self.iter().map(ToBorrowed::to_borrowed).collect()
    }
}

impl<T: ToOwned> ToOwned for Box<[T]> {
    type Owned = Box<[T::Owned]>;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        self.iter().map(ToOwned::to_owned).collect()
    }
}

impl<T: IntoOwned> IntoOwned for Box<[T]> {
    type Owned = Box<[T::Owned]>;

    #[inline]
    fn into_owned(self) -> Self::Owned {
        self.into_vec()
            .into_iter()
            .map(IntoOwned::into_owned)
            .collect()
    }
}

// BTreeMap

impl<'a, K, V> ToBorrowed<'a> for BTreeMap<K, V>
where
    K: ToBorrowed<'a> + Ord,
    V: ToBorrowed<'a>,
{
    #[inline]
    fn to_borrowed(&'a self) -> Self {
        self.iter()
            .map(|(k, v)| (ToBorrowed::to_borrowed(k), ToBorrowed::to_borrowed(v)))
            .collect()
    }
}

impl<K, V> ToOwned for BTreeMap<K, V>
where
    K: ToOwned,
    <K as ToOwned>::Owned: Ord,
    V: ToOwned,
{
    type Owned = BTreeMap<K::Owned, V::Owned>;

    #[inline]
    fn to_owned(&self) -> Self::Owned where {
        self.iter()
            .map(|(k, v)| (ToOwned::to_owned(k), ToOwned::to_owned(v)))
            .collect()
    }
}

impl<K, V> IntoOwned for BTreeMap<K, V>
where
    K: IntoOwned,
    <K as IntoOwned>::Owned: Ord,
    V: IntoOwned,
{
    type Owned = BTreeMap<K::Owned, V::Owned>;

    #[inline]
    fn into_owned(self) -> Self::Owned {
        self.into_iter()
            .map(|(k, v)| (IntoOwned::into_owned(k), IntoOwned::into_owned(v)))
            .collect()
    }
}
