//! `Mul` implementation for the `Transform` enum for 2D arrays.

use ndarray::{Array, ArrayBase, Data, Ix2, s};
use std::ops::Mul;

use crate::Transform;

impl<S, A> Mul<ArrayBase<S, Ix2>> for Transform
where
    S: Data<Elem = A>,
    A: Clone,
{
    type Output = Array<A, Ix2>;

    #[inline]
    fn mul(self, rhs: ArrayBase<S, Ix2>) -> Self::Output {
        let v = rhs.view();
        match self {
            Self::Identity => v.to_owned(),
            Self::Rotate90 => v.t().slice(s![.., ..;-1]).to_owned(),
            Self::Rotate180 => v.slice(s![..;-1, ..;-1]).to_owned(),
            Self::Rotate270 => v.t().slice(s![..;-1, ..]).to_owned(),
            Self::FlipHorizontal => v.slice(s![.., ..;-1]).to_owned(),
            Self::FlipVertical => v.slice(s![..;-1, ..]).to_owned(),
            Self::FlipDiagonal => v.t().to_owned(),
            Self::FlipAntiDiagonal => v.slice(s![..;-1, ..;-1]).t().to_owned(),
        }
    }
}

impl<'a, S, A> Mul<&'a ArrayBase<S, Ix2>> for Transform
where
    S: Data<Elem = A>,
    A: Clone,
{
    type Output = Array<A, Ix2>;

    #[inline]
    fn mul(self, rhs: &'a ArrayBase<S, Ix2>) -> Self::Output {
        self * rhs.view()
    }
}
