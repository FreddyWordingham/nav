use nav::{ALL_DIRECTIONS, Direction};

#[test]
fn test_direction_is_vertical() {
    assert!(Direction::North.is_vertical());
    assert!(!Direction::East.is_vertical());
    assert!(Direction::South.is_vertical());
    assert!(!Direction::West.is_vertical());
}

#[test]
fn test_direction_is_horizontal() {
    assert!(!Direction::North.is_horizontal());
    assert!(Direction::East.is_horizontal());
    assert!(!Direction::South.is_horizontal());
    assert!(Direction::West.is_horizontal());
}

#[test]
fn test_direction_categories_are_exhaustive() {
    // Each direction must belong to exactly one category
    for direction in ALL_DIRECTIONS {
        let categories = [direction.is_vertical(), direction.is_horizontal()];

        assert_eq!(
            categories.iter().filter(|&&x| x).count(),
            1,
            "Direction {:?} should belong to exactly one category",
            direction
        );
    }
}
