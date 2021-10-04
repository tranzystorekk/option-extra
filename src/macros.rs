/// Converts any enum to [`Option`].
///
/// Makes a [`Some`] from a selected variant of your enum.
///
/// General syntax:
///
/// ```man
/// some!( if let <enum variant> [{ <ident>... }] = <expr> [, when <guard expr>] [=> <then expr>] )
/// ```
///
/// where `<ident>` is `<name> [:]`.
///
/// Currently, until compile-time reflection becomes a thing,
/// you need to specify bindings when there are multiple fields in your variant:
///
/// ```ignore
/// enum MyEnum {
///     One(i32),
///     Many(i32, i32, i32)
/// }
///
/// some!(if let MyEnum::One = <expr>);
/// some!(if let MyEnum::Many = <expr>); // fails
/// some!(if let MyEnum::Many { a, b, c } = <expr>);
/// ```
///
/// # Examples
///
/// Short version for one-element variants:
///
/// ```
/// use option_extra::some;
///
/// enum MyEnum {
///     Int(i32),
///     Other,
/// }
///
/// let int = MyEnum::Int(1);
/// let other = MyEnum::Other;
///
/// assert_eq!(some!(if let MyEnum::Int = int), Some(1));
/// assert_eq!(some!(if let MyEnum::Int = other), None);
/// ```
///
/// Pairs perfectly with [`Iterator`] methods like [`filter_map`](Iterator::filter_map):
///
/// ```
/// use option_extra::some;
///
/// enum MyEnum {
///     A(i32, bool),
///     B(u8),
///     C
/// }
///
/// use MyEnum::*;
///
/// let v = vec![A(10, true), B(0), C, B(1), A(4, false), C];
/// let a_only: Vec<_> = v.into_iter().filter_map(|el| some!(if let A {n, b} = el)).collect();
///
/// assert_eq!(a_only, [(10, true), (4, false)]);
/// ```
///
/// Works with tuple variants:
///
/// ```
/// use option_extra::some;
///
/// enum MyEnum {
///     Variant(i32, bool),
///     Other,
/// }
///
/// let v = MyEnum::Variant(10, true);
///
/// assert_eq!(some!(if let MyEnum::Variant {n, b} = v), Some((10, true)));
/// ```
///
/// Or with struct variants (when suffixing field names with colons):
///
/// ```
/// use option_extra::some;
///
/// enum MyEnum {
///     Struct {
///         id: u16,
///         name: &'static str,
///     },
///     Other,
/// }
///
/// let s = MyEnum::Struct {
///     id: 20,
///     name: "abcd",
/// };
///
/// assert_eq!(some!(if let MyEnum::Struct {id:, name:} = s), Some((20, "abcd")));
/// ```
///
/// Optionally add an expression to which the wrapped value will be mapped:
///
/// ```
/// use option_extra::some;
///
/// enum MyEnum {
///     Int(i32),
///     Bool(bool),
/// }
///
/// let v_int = MyEnum::Int(10);
/// let v_bool = MyEnum::Bool(true);
///
/// assert_eq!(some!(if let MyEnum::Int { n } = v_int => (n, n + 1)), Some((10, 11)));
/// assert_eq!(some!(if let MyEnum::Int { n } = v_bool => (n, n + 1)), None);
/// ```
///
/// You can also add guards to further constrain which wrapped values are allowed:
///
/// ```
/// use option_extra::some;
///
/// enum MyEnum {
///     Val(i32),
///     Name(String),
/// }
///
/// let v = MyEnum::Val(10);
/// let v_odd = MyEnum::Val(15);
///
/// assert_eq!(some!(if let MyEnum::Val {x} = v, when x % 2 == 0), Some(10));
/// assert_eq!(some!(if let MyEnum::Val {x} = v_odd, when x % 2 == 0), None);
/// ```
#[macro_export]
macro_rules! some {
    ( if let $p:path = $x:expr ) => {
        match $x {
            $p(inner) => ::std::option::Option::Some(inner),
            _ => ::std::option::Option::None,
        }
    };

    ( if let $p:path {$($n:ident),+} = $x:expr $(, when $guard:expr)? ) => {
        match $x {
            $p($($n),+) $(if $guard)? => ::std::option::Option::Some(($($n),+)),
            _ => ::std::option::Option::None,
        }
    };

    ( if let $p:path {$($n:ident),+} = $x:expr $(, when $guard:expr)? => $then:expr ) => {
        match $x {
            $p($($n),+) $(if $guard)? => ::std::option::Option::Some($then),
            _ => ::std::option::Option::None,
        }
    };

    ( if let $p:path {$($n:ident:),+} = $x:expr $(, when $guard:expr)? ) => {
        match $x {
            $p{$($n),+} $(if $guard)? => ::std::option::Option::Some(($($n),+)),
            _ => ::std::option::Option::None,
        }
    };

    ( if let $p:path {$($n:ident:),+} = $x:expr $(, when $guard:expr)? => $then:expr ) => {
        match $x {
            $p{$($n),+} $(if $guard)? => ::std::option::Option::Some($then),
            _ => ::std::option::Option::None,
        }
    };
}
