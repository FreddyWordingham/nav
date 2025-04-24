use nav::Transform;

#[test]
fn transform_to_u8() {
    let i: u8 = Transform::Identity.into();
    let r: u8 = Transform::Rotate90.into();
    let u: u8 = Transform::Rotate180.into();
    let l: u8 = Transform::Rotate270.into();
    let h: u8 = Transform::FlipHorizontal.into();
    let d: u8 = Transform::FlipDiagonal.into();
    let v: u8 = Transform::FlipVertical.into();
    let a: u8 = Transform::FlipAntiDiagonal.into();
    assert_eq!(i, 0);
    assert_eq!(r, 1);
    assert_eq!(u, 2);
    assert_eq!(l, 3);
    assert_eq!(h, 4);
    assert_eq!(d, 5);
    assert_eq!(v, 6);
    assert_eq!(a, 7);
}

#[test]
fn transform_from_u8() {
    assert_eq!(Transform::try_from(0).unwrap(), Transform::Identity);
    assert_eq!(Transform::try_from(1).unwrap(), Transform::Rotate90);
    assert_eq!(Transform::try_from(2).unwrap(), Transform::Rotate180);
    assert_eq!(Transform::try_from(3).unwrap(), Transform::Rotate270);
    assert_eq!(Transform::try_from(4).unwrap(), Transform::FlipHorizontal);
    assert_eq!(Transform::try_from(5).unwrap(), Transform::FlipDiagonal);
    assert_eq!(Transform::try_from(6).unwrap(), Transform::FlipVertical);
    assert_eq!(Transform::try_from(7).unwrap(), Transform::FlipAntiDiagonal);
}
