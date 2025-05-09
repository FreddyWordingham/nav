//! `Display` implementation for the `Transform` enum.

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::Transform;

impl Display for Transform {
    #[inline]
    #[expect(clippy::min_ident_chars, reason = "This is a common pattern in Rust.")]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            Self::Identity => write!(f, "I"),
            Self::Rotate90 => write!(f, "R"),
            Self::Rotate180 => write!(f, "U"),
            Self::Rotate270 => write!(f, "L"),
            Self::FlipHorizontal => write!(f, "|"),
            Self::FlipDiagonal => write!(f, "/"),
            Self::FlipVertical => write!(f, "-"),
            Self::FlipAntiDiagonal => write!(f, "\\"),
        }
    }
}
