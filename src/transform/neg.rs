//! `Neg` implementation for the `Transform` enum.

use std::ops::Neg;

use crate::Transform;

impl Neg for Transform {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Self::Identity => Self::Identity,
            Self::Rotate90 => Self::Rotate270,
            Self::Rotate180 => Self::Rotate180,
            Self::Rotate270 => Self::Rotate90,
            Self::FlipHorizontal => Self::FlipHorizontal,
            Self::FlipDiagonal => Self::FlipDiagonal,
            Self::FlipVertical => Self::FlipVertical,
            Self::FlipAntiDiagonal => Self::FlipAntiDiagonal,
        }
    }
}
