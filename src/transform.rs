//! ## `transform`
//!
//! The `transform` module provides the `Transform` enum, which represents the dihedral group D4.

#[cfg(feature = "array")]
use ndarray::{Array, ArrayBase, Data, Ix2, s};
use num_enum::{IntoPrimitive, TryFromPrimitive};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Mul, MulAssign, Neg},
    str::FromStr,
};

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
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
    #[must_use]
    pub fn is_identity(self) -> bool {
        self == Self::Identity
    }

    /// Returns true if the `Transform` is a rotation.
    #[must_use]
    pub const fn is_rotation(self) -> bool {
        matches!(self, Self::Rotate90 | Self::Rotate180 | Self::Rotate270)
    }

    /// Returns true if the `Transform` is a flip.
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

impl MulAssign for Transform {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[cfg(feature = "array")]
impl<S, A> Mul<ArrayBase<S, Ix2>> for Transform
where
    S: Data<Elem = A>,
    A: Clone,
{
    type Output = Array<A, Ix2>;

    fn mul(self, rhs: ArrayBase<S, Ix2>) -> Self::Output {
        let v = rhs.view();
        match self {
            Self::Identity => v.to_owned(),
            Self::Rotate90 => v.t().slice(s![.., ..;-1]).to_owned(),
            Self::Rotate180 => v.slice(s![..;-1, ..;-1]).to_owned(),
            Self::Rotate270 => v.t().slice(s![..;-1, ..]).to_owned(),
            Self::FlipHorizontal => v.slice(s![.., ..;-1]).to_owned(),
            Self::FlipVertical => v.slice(s![..;-1, ..]).to_owned(),
            Self::FlipDiagonal => v.t().to_owned(),
            Self::FlipAntiDiagonal => v.slice(s![..;-1, ..;-1]).t().to_owned(),
        }
    }
}

#[cfg(feature = "array")]
impl<'a, S, A> Mul<&'a ArrayBase<S, Ix2>> for Transform
where
    S: Data<Elem = A>,
    A: Clone,
{
    type Output = Array<A, Ix2>;

    fn mul(self, rhs: &'a ArrayBase<S, Ix2>) -> Self::Output {
        self * rhs.view()
    }
}

impl Neg for Transform {
    type Output = Self;

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

impl FromStr for Transform {
    type Err = &'static str;

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
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        match *self {
            Self::Identity => write!(fmt, "I"),
            Self::Rotate90 => write!(fmt, "R"),
            Self::Rotate180 => write!(fmt, "U"),
            Self::Rotate270 => write!(fmt, "L"),
            Self::FlipHorizontal => write!(fmt, "|"),
            Self::FlipDiagonal => write!(fmt, "/"),
            Self::FlipVertical => write!(fmt, "-"),
            Self::FlipAntiDiagonal => write!(fmt, "\\"),
        }
    }
}
