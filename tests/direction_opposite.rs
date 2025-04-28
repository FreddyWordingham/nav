use nav::Direction;

#[test]
fn test_direction_neg() {
    assert_eq!(-Direction::North, Direction::South);
    assert_eq!(-Direction::East, Direction::West);
    assert_eq!(-Direction::South, Direction::North);
    assert_eq!(-Direction::West, Direction::East);
}
