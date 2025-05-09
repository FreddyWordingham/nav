//! ## `transform`
//!
//! The `transform` module provides the `Transform` enum, which represents the dihedral group D4.

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "array")]
mod arr2;
mod display;
mod from_str;
mod mul;
mod neg;

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
