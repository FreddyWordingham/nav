//! ## `transform`
//!
//! The `transform` module provides the `Transform` enum, which represents the dihedral group D4.

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Mul,
    str::FromStr,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// All transformations in their order of definition.
pub const ALL_TRANSFORMS: [Transform; 8] = [
    Transform::Identity,
    Transform::Rotate90,
    Transform::Rotate180,
    Transform::Rotate270,
    Transform::FlipHorizontal,
    Transform::FlipDiagonal,
    Transform::FlipVertical,
    Transform::FlipAntiDiagonal,
];

/// The eight transformations that can be applied to a 2D grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
#[expect(clippy::exhaustive_enums, reason = "This enum is exhaustive and will not be extended.")]
pub enum Transform {
    /// No transformation:                  (x, y) -> ( x,  y)
    Identity = 0,
    /// Rotate 90 degrees clockwise:        (x, y) -> ( y, -x)
    Rotate90 = 1,
    /// One half turn:                      (x, y) -> (-x, -y)
    Rotate180 = 2,
    /// Rotate 270 degrees clockwise:       (x, y) -> (-y,  x)
    Rotate270 = 3,
    /// Flip around the vertical axis:      (x, y) -> (-x,  y)
    FlipHorizontal = 4,
    /// Flip around the main diagonal:      (x, y) -> ( y,  x)
    FlipDiagonal = 5,
    /// Flip around the horizontal axis:    (x, y) -> ( x, -y)
    FlipVertical = 6,
    /// Flip around the anti-diagonal:      (x, y) -> (-y, -x)
    FlipAntiDiagonal = 7,
}

impl Transform {
    /// Returns true if the `Transform` is the `Identity` `Transform`.
    #[inline]
    #[must_use]
    pub fn is_identity(self) -> bool {
        self == Self::Identity
    }

    /// Returns true if the `Transform` is a rotation.
    #[inline]
    #[must_use]
    pub const fn is_rotation(self) -> bool {
        matches!(self, Self::Rotate90 | Self::Rotate180 | Self::Rotate270)
    }

    /// Returns true if the `Transform` is a flip.
    #[inline]
    #[must_use]
    pub const fn is_flip(self) -> bool {
        matches!(
            self,
            Self::FlipHorizontal | Self::FlipDiagonal | Self::FlipVertical | Self::FlipAntiDiagonal
        )
    }
}

impl Mul for Transform {
    type Output = Self;

    #[inline]
    #[expect(clippy::min_ident_chars, reason = "The names `a` and `b` are used as math variables.")]
    #[expect(
        clippy::unwrap_used,
        reason = "The math guarantees that the result will never trigger the unwrap."
    )]
    fn mul(self, rhs: Self) -> Self::Output {
        let a: u8 = self.into();
        let b: u8 = rhs.into();

        // Determine if each is a flip (IDs 4–7) and extract the rotation exponent
        let (is_flip_a, k_a) = (a >= 4, a % 4);
        let (is_flip_b, k_b) = (b >= 4, b % 4);

        let id = match (is_flip_a, is_flip_b) {
            (false, false) => (k_a + k_b) % 4,          // r^i * r^j = r^(i+j)
            (false, true) => 4 + ((k_a + k_b) % 4),     // r^i * (r^j f) = r^(i+j) f
            (true, false) => 4 + ((k_a + 4 - k_b) % 4), // (r^i f) * r^j = r^(i-j) f
            (true, true) => (k_a + 4 - k_b) % 4,        // (r^i f) * (r^j f) = r^(i-j)
        };

        Self::try_from(id).unwrap()
    }
}

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
