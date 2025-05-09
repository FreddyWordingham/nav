//! `FromStr` implementation for the `Transform` enum.

use std::str::FromStr;

use crate::Transform;

impl FromStr for Transform {
    type Err = &'static str;

    #[inline]
    #[expect(clippy::min_ident_chars, reason = "The name `s` clearly refers to a string here.")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "I" => Ok(Self::Identity),
            "R" => Ok(Self::Rotate90),
            "U" => Ok(Self::Rotate180),
            "L" => Ok(Self::Rotate270),
            "|" => Ok(Self::FlipHorizontal),
            "/" => Ok(Self::FlipDiagonal),
            "-" => Ok(Self::FlipVertical),
            "\\" => Ok(Self::FlipAntiDiagonal),
            _ => Err("Invalid transform, expected one of: I, R, U, L, |, /, -, \\"),
        }
    }
}
