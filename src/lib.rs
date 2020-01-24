//! Reliably remove a directory and all of its children.
//!
//! This library provides a reliable implementation of `remove_dir_all` for Windows.
//! For Unix systems, it re-exports `std::fs::remove_dir_all`.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

#[cfg(test)]
#[macro_use]
extern crate doc_comment;

#[cfg(test)]
doctest!("../README.md");

#[cfg(windows)]
mod fs;

#[cfg(windows)]
pub use self::fs::remove_dir_all;

#[cfg(not(windows))]
pub use std::fs::remove_dir_all;
