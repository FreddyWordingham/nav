use nav::{ALL_DIRECTIONS, Direction};
use std::str::FromStr;

#[test]
fn test_direction_to_string() {
    assert_eq!(Direction::North.to_string(), "N");
    assert_eq!(Direction::East.to_string(), "E");
    assert_eq!(Direction::South.to_string(), "S");
    assert_eq!(Direction::West.to_string(), "W");

    assert_eq!(format!("{}", Direction::North), "N");
    assert_eq!(format!("{}", Direction::East), "E");
    assert_eq!(format!("{}", Direction::South), "S");
    assert_eq!(format!("{}", Direction::West), "W");
}

#[test]
fn test_direction_from_str() {
    // Test short forms
    assert_eq!(Direction::from_str("N").unwrap(), Direction::North);
    assert_eq!(Direction::from_str("E").unwrap(), Direction::East);
    assert_eq!(Direction::from_str("S").unwrap(), Direction::South);
    assert_eq!(Direction::from_str("W").unwrap(), Direction::West);

    // Test full forms
    assert_eq!(Direction::from_str("North").unwrap(), Direction::North);
    assert_eq!(Direction::from_str("East").unwrap(), Direction::East);
    assert_eq!(Direction::from_str("South").unwrap(), Direction::South);
    assert_eq!(Direction::from_str("West").unwrap(), Direction::West);

    // Test case insensitivity
    assert_eq!(Direction::from_str("n").unwrap(), Direction::North);
    assert_eq!(Direction::from_str("EAST").unwrap(), Direction::East);
    assert_eq!(Direction::from_str("south").unwrap(), Direction::South);
    assert_eq!(Direction::from_str("wEsT").unwrap(), Direction::West);
}

#[test]
fn test_direction_from_str_error() {
    assert!(Direction::from_str("").is_err());
    assert!(Direction::from_str("X").is_err());
    assert!(Direction::from_str("NorthEast").is_err());
    assert!(Direction::from_str("SouthWest").is_err());
    assert!(Direction::from_str("123").is_err());

    let err = Direction::from_str("Invalid").unwrap_err();
    assert_eq!(
        err,
        "Invalid direction, expected one of: N, E, S, W, North, East, South, West"
    );
}

#[test]
fn test_parse_method() {
    assert_eq!("N".parse::<Direction>().unwrap(), Direction::North);
    assert_eq!("east".parse::<Direction>().unwrap(), Direction::East);
    assert_eq!("SOUTH".parse::<Direction>().unwrap(), Direction::South);
    assert_eq!("West".parse::<Direction>().unwrap(), Direction::West);
}

#[test]
fn test_roundtrip_conversion() {
    for dir in ALL_DIRECTIONS {
        let s = dir.to_string();
        let parsed = Direction::from_str(&s).unwrap();
        assert_eq!(dir, parsed);
    }
}
