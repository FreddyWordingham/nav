use nav::{Direction, Transform};

#[test]
fn identity() {
    let table = [
        (Direction::North, Transform::Identity, Direction::North),
        (Direction::East, Transform::Identity, Direction::East),
        (Direction::South, Transform::Identity, Direction::South),
        (Direction::West, Transform::Identity, Direction::West),
    ];
    for (d, t, e) in table {
        assert_eq!(d * t, e);
    }
}

#[test]
fn rotate_90() {
    let table = [
        (Direction::North, Transform::Rotate90, Direction::East),
        (Direction::East, Transform::Rotate90, Direction::South),
        (Direction::South, Transform::Rotate90, Direction::West),
        (Direction::West, Transform::Rotate90, Direction::North),
    ];
    for (d, t, e) in table {
        assert_eq!(d * t, e);
    }
}

#[test]
fn rotate_180() {
    let table = [
        (Direction::North, Transform::Rotate180, Direction::South),
        (Direction::East, Transform::Rotate180, Direction::West),
        (Direction::South, Transform::Rotate180, Direction::North),
        (Direction::West, Transform::Rotate180, Direction::East),
    ];
    for (d, t, e) in table {
        assert_eq!(d * t, e);
    }
}

#[test]
fn rotate_270() {
    let table = [
        (Direction::North, Transform::Rotate270, Direction::West),
        (Direction::East, Transform::Rotate270, Direction::North),
        (Direction::South, Transform::Rotate270, Direction::East),
        (Direction::West, Transform::Rotate270, Direction::South),
    ];
    for (d, t, e) in table {
        assert_eq!(d * t, e);
    }
}

#[test]
fn flip_horizontal() {
    let table = [
        (Direction::North, Transform::FlipHorizontal, Direction::North),
        (Direction::East, Transform::FlipHorizontal, Direction::West),
        (Direction::South, Transform::FlipHorizontal, Direction::South),
        (Direction::West, Transform::FlipHorizontal, Direction::East),
    ];
    for (d, t, e) in table {
        assert_eq!(d * t, e);
    }
}

#[test]
fn flip_diagonal() {
    let table = [
        (Direction::North, Transform::FlipDiagonal, Direction::East),
        (Direction::East, Transform::FlipDiagonal, Direction::North),
        (Direction::South, Transform::FlipDiagonal, Direction::West),
        (Direction::West, Transform::FlipDiagonal, Direction::South),
    ];
    for (d, t, e) in table {
        assert_eq!(d * t, e);
    }
}

#[test]
fn flip_vertical() {
    let table = [
        (Direction::North, Transform::FlipVertical, Direction::South),
        (Direction::East, Transform::FlipVertical, Direction::East),
        (Direction::South, Transform::FlipVertical, Direction::North),
        (Direction::West, Transform::FlipVertical, Direction::West),
    ];
    for (d, t, e) in table {
        assert_eq!(d * t, e);
    }
}

#[test]
fn flip_anti_diagonal() {
    let table = [
        (Direction::North, Transform::FlipAntiDiagonal, Direction::West),
        (Direction::East, Transform::FlipAntiDiagonal, Direction::South),
        (Direction::South, Transform::FlipAntiDiagonal, Direction::East),
        (Direction::West, Transform::FlipAntiDiagonal, Direction::North),
    ];
    for (d, t, e) in table {
        assert_eq!(d * t, e);
    }
}
