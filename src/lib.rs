//! # `Nav`
//!
//! `Nav` is a very simple utility library providing types for working with cardinal directions and transformations.

#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::restriction)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![deny(dead_code)]
#![deny(future_incompatible)]
#![deny(improper_ctypes)]
#![deny(macro_use_extern_crate)]
#![deny(missing_docs)]
#![deny(nonstandard_style)]
#![deny(path_statements)]
#![deny(rust_2018_idioms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_code)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
#![deny(unused_assignments)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_must_use)]
#![deny(unused_mut)]
#![deny(unused_variables)]
#![deny(unused)]
#![deny(warnings)]
#![allow(clippy::arithmetic_side_effects, reason = "Too restrictive for this crate.")]
#![allow(clippy::blanket_clippy_restriction_lints, reason = "More lints are always better.")]
#![allow(clippy::implicit_return, reason = "Implicit returns are idiomatic in Rust.")]
#![allow(
    clippy::integer_division_remainder_used,
    reason = "This crate should not be considered suitable for cryptographic use."
)]
#![allow(
    clippy::pub_use,
    reason = "It is intended to expose the `Direction` and `Transform` types at the crate level."
)]

mod direction;
mod transform;

pub use direction::{ALL_DIRECTIONS, Direction};
pub use transform::{ALL_TRANSFORMS, Transform};
