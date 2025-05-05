//! ## `direction`
//!
//! The `direction` module provides the `Direction` enum, which represents the four cardinal directions: North, East, South, and West.

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Mul, MulAssign, Neg},
    str::FromStr,
};

use crate::Transform;

/// All cardinal directions in their order of definition.
pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West, //
];

/// The four cardinal directions: North, East, South, and West.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
#[expect(clippy::exhaustive_enums, reason = "This enum is exhaustive and will not be extended.")]
pub enum Direction {
    /// Upwards
    North = 0,
    /// Rightwards
    East = 1,
    /// Downwards
    South = 2,
    /// Leftwards
    West = 3,
}

impl Direction {
    /// Returns true if the `Direction` is `North` or `South`.
    #[inline]
    #[must_use]
    pub const fn is_vertical(self) -> bool {
        matches!(self, Self::North | Self::South)
    }

    /// Returns true if the `Direction` is `East` or `West`.
    #[inline]
    #[must_use]
    pub const fn is_horizontal(self) -> bool {
        matches!(self, Self::East | Self::West)
    }
}

impl Mul<Transform> for Direction {
    type Output = Self;

    /// Applies a `Transform` to the current `Direction`.
    ///
    /// # Panics
    ///
    /// This function uses `unwrap()` internally but will never panic because the
    /// transformation math guarantees that the result will always be a valid `Direction` value (0-3).
    #[inline]
    #[expect(clippy::min_ident_chars, reason = "The names `a` and `b` are used as math variables.")]
    #[expect(
        clippy::unwrap_used,
        reason = "The math guarantees that the result will never trigger the unwrap."
    )]
    fn mul(self, rhs: Transform) -> Self::Output {
        let v: u8 = self.into();
        Self::try_from(match rhs {
            Transform::Identity => v,
            Transform::Rotate90 => (v + 1) % 4,
            Transform::Rotate180 => (v + 2) % 4,
            Transform::Rotate270 => (v + 3) % 4,
            Transform::FlipHorizontal => (4 - v) % 4,
            Transform::FlipDiagonal => (5 - v) % 4,
            Transform::FlipVertical => (6 - v) % 4,
            Transform::FlipAntiDiagonal => (7 - v) % 4,
        })
        .unwrap()
    }
}
impl MulAssign<Transform> for Direction {
    /// Applies a `Transform` to the current `Direction`.
    #[inline]
    fn mul_assign(&mut self, rhs: Transform) {
        *self = *self * rhs;
    }
}

impl Neg for Direction {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

impl FromStr for Direction {
    type Err = &'static str;

    #[inline]
    #[expect(clippy::min_ident_chars, reason = "The variable `s` is clearly a string.")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" | "NORTH" => Ok(Self::North),
            "E" | "EAST" => Ok(Self::East),
            "S" | "SOUTH" => Ok(Self::South),
            "W" | "WEST" => Ok(Self::West),
            _ => Err("Invalid direction, expected one of: N, E, S, W, North, East, South, West"),
        }
    }
}

impl Display for Direction {
    #[inline]
    #[expect(clippy::min_ident_chars, reason = "This is a common pattern in Rust.")]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match *self {
            Self::North => write!(f, "N"),
            Self::East => write!(f, "E"),
            Self::South => write!(f, "S"),
            Self::West => write!(f, "W"),
        }
    }
}
