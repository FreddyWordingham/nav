use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::Transform;

/// The four cardinal directions: North, East, South, and West.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
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

/// All cardinal directions in their order of definition.
pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West, //
];

impl Direction {
    /// Applies a `Transform` to the current `Direction`.
    #[must_use]
    pub fn transform(self, transform: Transform) -> Self {
        let v: u8 = self.into();
        Direction::try_from(match transform {
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

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" | "NORTH" => Ok(Direction::North),
            "E" | "EAST" => Ok(Direction::East),
            "S" | "SOUTH" => Ok(Direction::South),
            "W" | "WEST" => Ok(Direction::West),
            _ => Err("Invalid direction, expected one of: N, E, S, W, North, East, South, West"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "N"),
            Direction::East => write!(f, "E"),
            Direction::South => write!(f, "S"),
            Direction::West => write!(f, "W"),
        }
    }
}
