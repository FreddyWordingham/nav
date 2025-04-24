//! # `Nav`
//!
//! `Nav` is a very simple utility library providing types for working with cardinal directions and transformations.

#![deny(warnings)]
#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(unreachable_code)]

mod direction;
mod transform;

pub use direction::{ALL_DIRECTIONS, Direction};
pub use transform::{ALL_TRANSFORMS, Transform};
