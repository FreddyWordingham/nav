use nav::{ALL_TRANSFORMS, Transform};

#[test]
fn test_transform_is_identity() {
    assert!(Transform::Identity.is_identity());
    assert!(!Transform::Rotate90.is_identity());
    assert!(!Transform::Rotate180.is_identity());
    assert!(!Transform::Rotate270.is_identity());
    assert!(!Transform::FlipHorizontal.is_identity());
    assert!(!Transform::FlipDiagonal.is_identity());
    assert!(!Transform::FlipVertical.is_identity());
    assert!(!Transform::FlipAntiDiagonal.is_identity());
}

#[test]
fn test_transform_is_rotation() {
    assert!(!Transform::Identity.is_rotation());
    assert!(Transform::Rotate90.is_rotation());
    assert!(Transform::Rotate180.is_rotation());
    assert!(Transform::Rotate270.is_rotation());
    assert!(!Transform::FlipHorizontal.is_rotation());
    assert!(!Transform::FlipDiagonal.is_rotation());
    assert!(!Transform::FlipVertical.is_rotation());
    assert!(!Transform::FlipAntiDiagonal.is_rotation());
}

#[test]
fn test_transform_is_flip() {
    assert!(!Transform::Identity.is_flip());
    assert!(!Transform::Rotate90.is_flip());
    assert!(!Transform::Rotate180.is_flip());
    assert!(!Transform::Rotate270.is_flip());
    assert!(Transform::FlipHorizontal.is_flip());
    assert!(Transform::FlipDiagonal.is_flip());
    assert!(Transform::FlipVertical.is_flip());
    assert!(Transform::FlipAntiDiagonal.is_flip());
}

#[test]
fn test_transform_categories_are_exhaustive() {
    // Each transform must belong to exactly one category
    for transform in ALL_TRANSFORMS {
        let categories = [transform.is_identity(), transform.is_rotation(), transform.is_flip()];

        assert_eq!(
            categories.iter().filter(|&&x| x).count(),
            1,
            "Transform {:?} should belong to exactly one category",
            transform
        );
    }
}
