use nav::{Direction, Transform};

const TEST_CASES: &[(Direction, Transform, Direction)] = &[
    // identity
    (Direction::North, Transform::Identity, Direction::North),
    (Direction::East, Transform::Identity, Direction::East),
    (Direction::South, Transform::Identity, Direction::South),
    (Direction::West, Transform::Identity, Direction::West),
    // rotate
    (Direction::North, Transform::Rotate90, Direction::East),
    (Direction::East, Transform::Rotate90, Direction::South),
    (Direction::South, Transform::Rotate90, Direction::West),
    (Direction::West, Transform::Rotate90, Direction::North),
    (Direction::North, Transform::Rotate180, Direction::South),
    (Direction::East, Transform::Rotate180, Direction::West),
    (Direction::South, Transform::Rotate180, Direction::North),
    (Direction::West, Transform::Rotate180, Direction::East),
    (Direction::North, Transform::Rotate270, Direction::West),
    (Direction::East, Transform::Rotate270, Direction::North),
    (Direction::South, Transform::Rotate270, Direction::East),
    (Direction::West, Transform::Rotate270, Direction::South),
    // flips
    (Direction::North, Transform::FlipHorizontal, Direction::North),
    (Direction::East, Transform::FlipHorizontal, Direction::West),
    (Direction::South, Transform::FlipHorizontal, Direction::South),
    (Direction::West, Transform::FlipHorizontal, Direction::East),
    (Direction::North, Transform::FlipVertical, Direction::South),
    (Direction::East, Transform::FlipVertical, Direction::East),
    (Direction::South, Transform::FlipVertical, Direction::North),
    (Direction::West, Transform::FlipVertical, Direction::West),
    (Direction::North, Transform::FlipDiagonal, Direction::East),
    (Direction::East, Transform::FlipDiagonal, Direction::North),
    (Direction::South, Transform::FlipDiagonal, Direction::West),
    (Direction::West, Transform::FlipDiagonal, Direction::South),
    (Direction::North, Transform::FlipAntiDiagonal, Direction::West),
    (Direction::East, Transform::FlipAntiDiagonal, Direction::South),
    (Direction::South, Transform::FlipAntiDiagonal, Direction::East),
    (Direction::West, Transform::FlipAntiDiagonal, Direction::North),
];

#[test]
fn all_transforms_mul() {
    for &(orig, transform, expected) in TEST_CASES {
        assert_eq!(orig * transform, expected, "failed: {:?} * {:?}", orig, transform);
    }
}

#[test]
fn all_transforms_mul_assign() {
    for &(orig, transform, expected) in TEST_CASES {
        let mut tmp = orig;
        tmp *= transform;
        assert_eq!(tmp, expected, "failed: {:?} *= {:?}", orig, transform);
    }
}
