use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    fmt::{Display, Formatter},
    ops::Mul,
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

impl Transform {
    /// Returns true if the `Transform` is the `Identity` `Transform`.
    pub fn is_identity(self) -> bool {
        self == Transform::Identity
    }

    /// Returns true if the `Transform` is a rotation.
    pub fn is_rotation(self) -> bool {
        matches!(self, Transform::Rotate90 | Transform::Rotate180 | Transform::Rotate270)
    }

    /// Returns true if the `Transform` is a flip.
    pub fn is_flip(self) -> bool {
        matches!(
            self,
            Transform::FlipHorizontal | Transform::FlipDiagonal | Transform::FlipVertical | Transform::FlipAntiDiagonal
        )
    }
}

impl Mul for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // Convert to raw IDs
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

        Transform::try_from(id).unwrap()
    }
}

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
