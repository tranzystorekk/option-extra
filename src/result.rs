/// Extra methods for the [`Result`] type.
pub trait ResultExt<T, E> {
    /// If `self` is [`Ok`], checks if the wrapped value satisfies the given predicate.
    /// Otherwise, returns `false`.
    ///
    /// Roughly equivalent to `matches!(&my_result, Ok(x) if predicate(x))`.
    ///
    /// # Examples
    ///
    /// ```
    /// use option_extra::ResultExt;
    ///
    /// assert!(Ok::<_, ()>(1).satisfies(|&n| n % 2 == 1));
    /// ```
    fn satisfies<P>(&self, predicate: P) -> bool
    where
        P: FnOnce(&T) -> bool;

    /// Mutates the wrapped `Ok` value with the given function
    /// and returns the resulting `self`.
    ///
    /// An `Err` is returned unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use option_extra::ResultExt;
    ///
    /// assert_eq!(Ok::<_, ()>(vec![1, 2, 3]).update(|v| v.push(0)), Ok(vec![1, 2, 3, 0]));
    /// ```
    fn update<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut T);
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn satisfies<P>(&self, predicate: P) -> bool
    where
        P: FnOnce(&T) -> bool,
    {
        match self {
            Ok(x) => predicate(x),
            _ => false,
        }
    }

    fn update<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut T),
    {
        match self {
            Ok(mut x) => {
                f(&mut x);
                Ok(x)
            }
            err => err,
        }
    }
}
