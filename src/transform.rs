use num_enum::{IntoPrimitive, TryFromPrimitive};

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
