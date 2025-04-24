use nav::{ALL_TRANSFORMS, Transform};
use std::str::FromStr;

#[test]
fn test_transform_to_string() {
    assert_eq!(Transform::Identity.to_string(), "I");
    assert_eq!(Transform::Rotate90.to_string(), "R");
    assert_eq!(Transform::Rotate180.to_string(), "U");
    assert_eq!(Transform::Rotate270.to_string(), "L");
    assert_eq!(Transform::FlipHorizontal.to_string(), "|");
    assert_eq!(Transform::FlipDiagonal.to_string(), "/");
    assert_eq!(Transform::FlipVertical.to_string(), "-");
    assert_eq!(Transform::FlipAntiDiagonal.to_string(), "\\");

    assert_eq!(format!("{}", Transform::Identity), "I");
    assert_eq!(format!("{}", Transform::Rotate90), "R");
    assert_eq!(format!("{}", Transform::Rotate180), "U");
    assert_eq!(format!("{}", Transform::Rotate270), "L");
    assert_eq!(format!("{}", Transform::FlipHorizontal), "|");
    assert_eq!(format!("{}", Transform::FlipDiagonal), "/");
    assert_eq!(format!("{}", Transform::FlipVertical), "-");
    assert_eq!(format!("{}", Transform::FlipAntiDiagonal), "\\");
}

#[test]
fn test_transform_from_str() {
    assert_eq!(Transform::from_str("I").unwrap(), Transform::Identity);
    assert_eq!(Transform::from_str("R").unwrap(), Transform::Rotate90);
    assert_eq!(Transform::from_str("U").unwrap(), Transform::Rotate180);
    assert_eq!(Transform::from_str("L").unwrap(), Transform::Rotate270);
    assert_eq!(Transform::from_str("|").unwrap(), Transform::FlipHorizontal);
    assert_eq!(Transform::from_str("/").unwrap(), Transform::FlipDiagonal);
    assert_eq!(Transform::from_str("-").unwrap(), Transform::FlipVertical);
    assert_eq!(Transform::from_str("\\").unwrap(), Transform::FlipAntiDiagonal);
}

#[test]
fn test_transform_from_str_error() {
    assert!(Transform::from_str("").is_err());
    assert!(Transform::from_str("Y").is_err());
    assert!(Transform::from_str("Identity").is_err());
    assert!(Transform::from_str("Diagonal").is_err());
    assert!(Transform::from_str("456").is_err());

    let err = Transform::from_str("Invalid").unwrap_err();
    assert_eq!(err, "Invalid transform, expected one of: I, R, U, L, |, /, -, \\");
}

#[test]
fn test_parse_method() {
    assert_eq!("I".parse::<Transform>().unwrap(), Transform::Identity);
    assert_eq!("r".parse::<Transform>().unwrap(), Transform::Rotate90);
    assert_eq!("-".parse::<Transform>().unwrap(), Transform::FlipVertical);
    assert_eq!("\\".parse::<Transform>().unwrap(), Transform::FlipAntiDiagonal);
}

#[test]
fn test_roundtrip_conversion() {
    for dir in ALL_TRANSFORMS {
        let s = dir.to_string();
        let parsed = Transform::from_str(&s).unwrap();
        assert_eq!(dir, parsed);
    }
}
