use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// The eight transformations that can be applied to a 2D grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Transform {
    /// No transformation:                  (x, y) -> ( x,  y).
    Identity = 0,
    /// Rotate 90 degrees clockwise:        (x, y) -> ( y, -x).
    Rotate90 = 1,
    /// One half turn:                      (x, y) -> (-x, -y).
    Rotate180 = 2,
    /// Rotate 270 degrees clockwise:       (x, y) -> (-y,  x).
    Rotate270 = 3,
    /// Flip around the vertical axis:      (x, y) -> (-x,  y).
    FlipHorizontal = 4,
    /// Flip around the main diagonal:      (x, y) -> ( y,  x).
    FlipDiagonal = 5,
    /// Flip around the horizontal axis:    (x, y) -> ( x, -y).
    FlipVertical = 6,
    /// Flip around the anti-diagonal:      (x, y) -> (-y, -x).
    FlipAntiDiagonal = 7,
}

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

impl FromStr for Transform {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "I" => Ok(Transform::Identity),
            "R" => Ok(Transform::Rotate90),
            "U" => Ok(Transform::Rotate180),
            "L" => Ok(Transform::Rotate270),
            "|" => Ok(Transform::FlipHorizontal),
            "/" => Ok(Transform::FlipDiagonal),
            "-" => Ok(Transform::FlipVertical),
            "\\" => Ok(Transform::FlipAntiDiagonal),
            _ => Err("Invalid transform, expected one of: I, R, U, L, |, /, -, \\"),
        }
    }
}

impl Display for Transform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Transform::Identity => write!(f, "I"),
            Transform::Rotate90 => write!(f, "R"),
            Transform::Rotate180 => write!(f, "U"),
            Transform::Rotate270 => write!(f, "L"),
            Transform::FlipHorizontal => write!(f, "|"),
            Transform::FlipDiagonal => write!(f, "/"),
            Transform::FlipVertical => write!(f, "-"),
            Transform::FlipAntiDiagonal => write!(f, "\\"),
        }
    }
}
