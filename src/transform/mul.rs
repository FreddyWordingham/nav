//! `Mul` implementation for the `Transform` enum.

use std::ops::{Mul, MulAssign};

use crate::Transform;

impl Mul for Transform {
    type Output = Self;

    #[inline]
    #[expect(clippy::min_ident_chars, reason = "The names `a` and `b` are used as math variables.")]
    #[expect(
        clippy::unwrap_used,
        reason = "The math guarantees that the result will never trigger the unwrap."
    )]
    fn mul(self, rhs: Self) -> Self::Output {
        let a: u8 = self.into();
        let b: u8 = rhs.into();

        // Determine if each is a flip (IDs 4–7) and extract the rotation exponent
        let (is_flip_a, k_a) = (a >= 4, a % 4);
        let (is_flip_b, k_b) = (b >= 4, b % 4);

        let id = match (is_flip_a, is_flip_b) {
            (false, false) => (k_a + k_b) % 4,          // r^i * r^j = r^(i+j)
            (false, true) => 4 + ((k_a + k_b) % 4),     // r^i * (r^j f) = r^(i+j) f
            (true, false) => 4 + ((k_a + 4 - k_b) % 4), // (r^i f) * r^j = r^(i-j) f
            (true, true) => (k_a + 4 - k_b) % 4,        // (r^i f) * (r^j f) = r^(i-j)
        };

        Self::try_from(id).unwrap()
    }
}

impl MulAssign for Transform {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
