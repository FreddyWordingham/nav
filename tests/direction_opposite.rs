use nav::Direction;

#[test]
fn test_direction_opposite() {
    assert_eq!(Direction::North.opposite(), Direction::South);
    assert_eq!(Direction::East.opposite(), Direction::West);
    assert_eq!(Direction::South.opposite(), Direction::North);
    assert_eq!(Direction::West.opposite(), Direction::East);
}
