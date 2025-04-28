<p align="center" style="font-size: 2.5em">
    Nav
</p>
<p align="center">
    <img src="./assets/icon.png" alt="Nav Icon" width="200" style="border-radius: 5%; border: 2px solid #000;">
</p>
<p align="center" style="font-size: 1.5em">
    A lightweight, minimal-dependency Rust library for handling cardinal directions and geometric transformations in 2D space.
</p>

[![crates.io](https://img.shields.io/crates/v/nav.svg)](https://crates.io/crates/nav)
[![Documentation](https://docs.rs/nav/badge.svg)](https://docs.rs/nav)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Cardinal Directions**: Complete representation of the four primary directions (North, East, South, West)
- **2D Transformations**: Full implementation of the dihedral group D4 (rotations and reflections)
- **Type Conversions**: Convert between string representations, enum values, and numeric types
- **Mathematical Operations**: Compose transformations and apply them to directions
- **No Standard Library Requirement**: Core-only implementation for embedded systems compatibility
- **Comprehensive Testing**: 100% test coverage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nav = "0.1.1"
```

## Usage

### Direction

The `Direction` enum represents the four cardinal directions: North, East, South, and West.

```rust
use nav::Direction;
use std::str::FromStr;

// Create directions
let north = Direction::North;
let east = Direction::East;

// Check direction properties
assert!(north.is_vertical());
assert!(east.is_horizontal());

// Get opposite direction
assert_eq!(-north, Direction::South);

// Convert to/from strings
assert_eq!(north.to_string(), "N");
assert_eq!(Direction::from_str("South").unwrap(), Direction::South);
assert_eq!("west".parse::<Direction>().unwrap(), Direction::West);

// Convert to/from numeric representation
let dir_value: u8 = north.into(); // 0
assert_eq!(Direction::try_from(2).unwrap(), Direction::South);
```

### Transform

The `Transform` enum represents the eight possible transformations in the dihedral group D4:

```rust
use nav::Transform;
use std::str::FromStr;

// Create transforms
let identity = Transform::Identity;
let rotate90 = Transform::Rotate90;
let flip_h = Transform::FlipHorizontal;

// Check transform properties
assert!(identity.is_identity());
assert!(rotate90.is_rotation());
assert!(flip_h.is_flip());

// Compose transforms
let rotate180 = rotate90 * rotate90;
assert_eq!(rotate180, Transform::Rotate180);

// Applying inverse transformations returns to identity
assert_eq!(flip_h * flip_h, Transform::Identity);

// Convert to/from strings
assert_eq!(rotate90.to_string(), "R");
assert_eq!(Transform::from_str("|").unwrap(), Transform::FlipHorizontal);
```

### Applying Transformations to Directions

You can apply transformations to directions using the multiplication operator:

```rust
use nav::{Direction, Transform};

// Apply rotations
assert_eq!(Direction::North * Transform::Rotate90, Direction::East);
assert_eq!(Direction::East * Transform::Rotate90, Direction::South);
assert_eq!(Direction::South * Transform::Rotate90, Direction::West);
assert_eq!(Direction::West * Transform::Rotate90, Direction::North);

// Apply reflections
assert_eq!(Direction::North * Transform::FlipHorizontal, Direction::North);
assert_eq!(Direction::East * Transform::FlipHorizontal, Direction::West);
```

### Working with All Values

The library provides constants for all directions and transforms:

```rust
use nav::{ALL_DIRECTIONS, ALL_TRANSFORMS, Direction, Transform};

// Iterate through all directions
for dir in ALL_DIRECTIONS {
    println!("Direction: {}", dir);
}

// Iterate through all transformations
for transform in ALL_TRANSFORMS {
    println!("Transform: {}", transform);
}
```

## Transform Symbols

The `Transform` enum uses single character symbols for string representation:

| Transform        | Symbol | Description                            | Effect on (x,y) |
| ---------------- | ------ | -------------------------------------- | --------------- |
| Identity         | I      | No transformation                      | (x, y)          |
| Rotate90         | R      | 90° clockwise rotation                 | (y, -x)         |
| Rotate180        | U      | 180° rotation                          | (-x, -y)        |
| Rotate270        | L      | 270° clockwise (90° counter-clockwise) | (-y, x)         |
| FlipHorizontal   | \|     | Reflection about vertical axis         | (-x, y)         |
| FlipDiagonal     | /      | Reflection about main diagonal         | (y, x)          |
| FlipVertical     | -      | Reflection about horizontal axis       | (x, -y)         |
| FlipAntiDiagonal | \\     | Reflection about anti-diagonal         | (-y, -x)        |

## Applications

This library is useful for:

- Game development (movement, rotation, grid-based operations)
- Cellular automata and grid-based simulations
- Procedural generation
- Puzzle solving algorithms
- Geometry processing

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
