//! Additional utilities for the [`Option`] type.
//!
//! New methods are added via the [`OptionExt`] trait:
//!
//! ```
//! use option_extra::OptionExt;
//! ```

/// Extra methods for the [`Option`] type.
pub trait OptionExt<T> {
    /// Like [`Option::zip`], but the other [`Option`] is obtained from `f`
    /// which is not evaluated if `self` is [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use option_extra::OptionExt;
    ///
    /// fn some() -> Option<i32> { Some(1) }
    /// fn none() -> Option<i32> { None }
    ///
    /// assert_eq!(Some("abc").zip_lazy(some), Some(("abc", 1)));
    /// assert_eq!(Some("abc").zip_lazy(none), None);
    /// assert_eq!(None::<i32>.zip_lazy(some), None);
    /// ```
    fn zip_lazy<U, F>(self, f: F) -> Option<(T, U)>
    where
        F: FnOnce() -> Option<U>;

    /// Checks if the wrapped value satisfies the given predicate,
    /// or returns `false` if `self` is [`None`].
    /// 
    /// ```
    /// use option_extra::OptionExt;
    /// 
    /// assert!(Some(1).satisfies(|&n| n % 2 == 1));
    /// ```
    fn satisfies<P>(&self, pred: P) -> bool
    where
        P: FnOnce(&T) -> bool;
}

impl<T> OptionExt<T> for Option<T> {
    fn zip_lazy<U, F>(self, f: F) -> Option<(T, U)>
    where
        F: FnOnce() -> Option<U>,
    {
        let a = self?;
        let b = f()?;

        Some((a, b))
    }

    fn satisfies<P>(&self, predicate: P) -> bool
    where
        P: FnOnce(&T) -> bool,
    {
        match self {
            Some(x) => predicate(x),
            None => false,
        }
    }
}
