//! Additional utilities for common [`std`] types.
//!
//! New methods are added via the traits:
//!
//! ```
//! use option_extra::OptionExt;
//! use option_extra::ResultExt;
//! ```
mod option;
mod result;

pub use option::OptionExt;
pub use result::ResultExt;
