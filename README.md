# option-extra

[![CI](https://github.com/tranzystorek-io/option-extra/actions/workflows/ci.yml/badge.svg)](https://github.com/tranzystorek-io/option-extra/actions/workflows/ci.yml)

## About

Additional utilities for common standard types, that feel missing.

## Usage

New methods live in the `OptionExt` / `ResultExt` traits:

```rust
use option_extra::OptionExt;

assert_eq!(Some(1).zip_lazy(|| Some("abcd")), Some((1, "abcd")));
```

```rust
use option_extra::ResultExt;

assert!(Ok::<_, ()>(1).satisfies(|&n| n % 2 == 1));
```
