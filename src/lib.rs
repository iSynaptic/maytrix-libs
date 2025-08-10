//! Root crate for the Maytrix workspace.
//!
//! This crate aggregates shared functionality and documentation for the
//! Maytrix project. See the workspace crates like `maytrix_value` and
//! `maytrix_domain` for concrete types.
//!
//! # Example
//!
//! Using a `Symbol` from the value crate:
//!
//! ```
//! use maytrix_value::Symbol;
//!
//! let s = Symbol::try_new("example_1").unwrap();
//! assert_eq!(s.as_str(), "example_1");
//! ```
