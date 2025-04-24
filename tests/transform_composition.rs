use nav::Transform;
use std::str::FromStr;

#[test]
fn test_identity_multiplication() {
    assert_eq!(Transform::Identity * Transform::Identity, Transform::Identity);

    assert_eq!(Transform::Identity * Transform::Rotate90, Transform::Rotate90);
    assert_eq!(Transform::Identity * Transform::Rotate180, Transform::Rotate180);
    assert_eq!(Transform::Identity * Transform::Rotate270, Transform::Rotate270);
    assert_eq!(Transform::Identity * Transform::FlipHorizontal, Transform::FlipHorizontal);
    assert_eq!(Transform::Identity * Transform::FlipDiagonal, Transform::FlipDiagonal);
    assert_eq!(Transform::Identity * Transform::FlipVertical, Transform::FlipVertical);
    assert_eq!(Transform::Identity * Transform::FlipAntiDiagonal, Transform::FlipAntiDiagonal);

    assert_eq!(Transform::Rotate90 * Transform::Identity, Transform::Rotate90);
    assert_eq!(Transform::Rotate180 * Transform::Identity, Transform::Rotate180);
    assert_eq!(Transform::Rotate270 * Transform::Identity, Transform::Rotate270);
    assert_eq!(Transform::FlipHorizontal * Transform::Identity, Transform::FlipHorizontal);
    assert_eq!(Transform::FlipDiagonal * Transform::Identity, Transform::FlipDiagonal);
    assert_eq!(Transform::FlipVertical * Transform::Identity, Transform::FlipVertical);
    assert_eq!(Transform::FlipAntiDiagonal * Transform::Identity, Transform::FlipAntiDiagonal);
}

#[test]
fn test_rotation_composition() {
    assert_eq!(Transform::Rotate90 * Transform::Rotate90, Transform::Rotate180);
    assert_eq!(Transform::Rotate90 * Transform::Rotate180, Transform::Rotate270);
    assert_eq!(Transform::Rotate90 * Transform::Rotate270, Transform::Identity);

    assert_eq!(Transform::Rotate180 * Transform::Rotate90, Transform::Rotate270);
    assert_eq!(Transform::Rotate180 * Transform::Rotate180, Transform::Identity);
    assert_eq!(Transform::Rotate180 * Transform::Rotate270, Transform::Rotate90);

    assert_eq!(Transform::Rotate270 * Transform::Rotate90, Transform::Identity);
    assert_eq!(Transform::Rotate270 * Transform::Rotate180, Transform::Rotate90);
    assert_eq!(Transform::Rotate270 * Transform::Rotate270, Transform::Rotate180);
}

#[test]
fn test_flip_composition() {
    assert_eq!(Transform::FlipHorizontal * Transform::FlipHorizontal, Transform::Identity);
    assert_eq!(Transform::FlipVertical * Transform::FlipVertical, Transform::Identity);
    assert_eq!(Transform::FlipDiagonal * Transform::FlipDiagonal, Transform::Identity);
    assert_eq!(Transform::FlipAntiDiagonal * Transform::FlipAntiDiagonal, Transform::Identity);

    assert_eq!(Transform::FlipHorizontal * Transform::FlipVertical, Transform::Rotate180);
    assert_eq!(Transform::FlipVertical * Transform::FlipHorizontal, Transform::Rotate180);

    assert_eq!(Transform::FlipDiagonal * Transform::FlipAntiDiagonal, Transform::Rotate180);
    assert_eq!(Transform::FlipAntiDiagonal * Transform::FlipDiagonal, Transform::Rotate180);
}

#[test]
fn test_mixed_composition() {
    assert_eq!(Transform::Rotate90 * Transform::FlipHorizontal, Transform::FlipDiagonal);
    assert_eq!(Transform::Rotate90 * Transform::FlipVertical, Transform::FlipAntiDiagonal);

    assert_eq!(Transform::FlipHorizontal * Transform::Rotate90, Transform::FlipAntiDiagonal);
    assert_eq!(Transform::FlipVertical * Transform::Rotate90, Transform::FlipDiagonal);

    assert_eq!(Transform::Rotate180 * Transform::FlipDiagonal, Transform::FlipAntiDiagonal);
    assert_eq!(Transform::FlipDiagonal * Transform::Rotate180, Transform::FlipAntiDiagonal);
}

#[test]
fn test_triple_composition() {
    assert_eq!(
        Transform::Rotate90 * (Transform::Rotate90 * Transform::Rotate90),
        (Transform::Rotate90 * Transform::Rotate90) * Transform::Rotate90
    );

    assert_eq!(
        Transform::FlipHorizontal * (Transform::Rotate90 * Transform::FlipVertical),
        (Transform::FlipHorizontal * Transform::Rotate90) * Transform::FlipVertical
    );
}

#[test]
fn test_all_compositions() {
    #[rustfmt::skip]
    const EXPECTED: [[&str; 8]; 8] = [
        // I   R    U    L    |    /    -    \
        ["I", "R", "U", "L", "|", "/", "-", "\\"],  // I
        ["R", "U", "L", "I", "/", "-", "\\","|"],   // R
        ["U", "L", "I", "R", "-", "\\","|", "/"],   // U
        ["L", "I", "R", "U", "\\","|", "/", "-"],   // L
        ["|", "\\","-", "/", "I", "L", "U", "R"],   // |
        ["/", "|", "\\","-", "R", "I", "L", "U"],   // /
        ["-", "/", "|", "\\","U", "R", "I", "L"],   // -
        ["\\","-", "/", "|", "L", "U", "R", "I"],   // \
    ];

    let all_transforms = [
        Transform::Identity,
        Transform::Rotate90,
        Transform::Rotate180,
        Transform::Rotate270,
        Transform::FlipHorizontal,
        Transform::FlipDiagonal,
        Transform::FlipVertical,
        Transform::FlipAntiDiagonal,
    ];

    for (i, &a) in all_transforms.iter().enumerate() {
        for (j, &b) in all_transforms.iter().enumerate() {
            assert_eq!(
                a * b,
                Transform::from_str(EXPECTED[i][j]).unwrap(),
                "Failed: {:?} * {:?}",
                a,
                b
            );
        }
    }
}
