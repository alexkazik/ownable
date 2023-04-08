use crate::traits::{IntoOwned, ToBorrowed, ToOwned};
use beef::generic::{Beef, Capacity, Cow};

// Cow

#[cfg_attr(docsrs, doc(cfg(feature = "beef")))]
impl<'a, T: ?Sized + alloc::borrow::ToOwned + Beef, U: Capacity> ToBorrowed<'a> for Cow<'a, T, U> {
    #[inline]
    fn to_borrowed(&'a self) -> Self {
        Cow::borrowed(self.as_ref())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "beef")))]
impl<T: ?Sized + alloc::borrow::ToOwned + 'static + Beef, U: Capacity> ToOwned for Cow<'_, T, U> {
    type Owned = Cow<'static, T, U>;

    #[inline]
    fn to_owned(&self) -> Cow<'static, T, U> {
        Cow::owned(T::to_owned(self.as_ref()))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "beef")))]
impl<T: ?Sized + alloc::borrow::ToOwned + 'static + Beef, U: Capacity> IntoOwned for Cow<'_, T, U> {
    type Owned = Cow<'static, T, U>;

    #[inline]
    fn into_owned(self) -> Cow<'static, T, U> {
        Cow::owned(self.into_owned())
    }
}
