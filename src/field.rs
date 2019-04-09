//! Field arithmetic for the doppio curve.
//!
//! Because doppio is intended for use in ristretto255-based proof
//! systems, the scalar field of ristretto255 is the *ground field*
//! for doppio.
//!
//! This implementation is derived from the 52-bit scalar
//! implementation contributed to `curve25519-dalek` by Andrew Moon.

use std::default::Default;
use std::ops::{Add, Mul, Neg, Sub};

use crate::Ristretto255Scalar;

/// Edwards `d` value, equal to `-86649/86650 mod p`.
/// TODO: actually generate the right value here
pub(crate) const EDWARDS_D: FieldElement = FieldElement([0; 5]);

/// A field element modulo \\(2\^{252} +
/// 27742317777372353535851937790883648493\\), the ground field for
/// the doppio curve and the scalar field for the ristretto255 group.
#[derive(Copy, Clone, Debug)]
pub struct FieldElement([u64; 5]);

impl Add<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: FieldElement) -> FieldElement {
        unimplemented!();
    }
}

impl Sub<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: FieldElement) -> FieldElement {
        unimplemented!();
    }
}

impl Mul<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> FieldElement {
        unimplemented!();
    }
}

impl Default for FieldElement {
    fn default() -> FieldElement {
        FieldElement::zero()
    }
}

impl From<Ristretto255Scalar> for FieldElement {
    fn from(packed: Ristretto255Scalar) -> FieldElement {
        unimplemented!();
    }
}

impl Into<Ristretto255Scalar> for FieldElement {
    fn into(self) -> Ristretto255Scalar {
        unimplemented!();
    }
}

impl Neg for FieldElement {
    type Output = Self;

    fn neg(self) -> Self {
        unimplemented!()
    }
}

impl FieldElement {
    pub fn zero() -> FieldElement {
        FieldElement([0; 5])
    }

    pub fn one() -> FieldElement {
        // This needs to return 1/R mod l
        unimplemented!();
    }

    pub fn invert(&self) -> FieldElement {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
