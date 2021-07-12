/// Extra methods for the [`Result`] type.
pub trait ResultExt<T, E> {
    /// If `self` is [`Ok`], checks if the wrapped value satisfies the given predicate.
    /// Otherwise, returns `false`.
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
}
