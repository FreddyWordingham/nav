use nav::Direction;

#[test]
fn direction_to_u8() {
    let n: u8 = Direction::North.into();
    let e: u8 = Direction::East.into();
    let s: u8 = Direction::South.into();
    let w: u8 = Direction::West.into();
    assert_eq!(n, 0);
    assert_eq!(e, 1);
    assert_eq!(s, 2);
    assert_eq!(w, 3);
}

#[test]
fn direction_from_u8() {
    assert_eq!(Direction::try_from(0).unwrap(), Direction::North);
    assert_eq!(Direction::try_from(1).unwrap(), Direction::East);
    assert_eq!(Direction::try_from(2).unwrap(), Direction::South);
    assert_eq!(Direction::try_from(3).unwrap(), Direction::West);
}
