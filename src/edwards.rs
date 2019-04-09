#![allow(non_snake_case)]

use crate::field::{FieldElement, EDWARDS_D};
use core::ops::{Add, Neg, Sub};

// ------------------------------------------------------------------------
// Internal point representations
// ------------------------------------------------------------------------

/// An `EdwardsPoint` represents a point on the Edwards form of the Doppio curve.
#[derive(Copy, Clone)]
#[allow(missing_docs)]
pub struct EdwardsPoint {
    pub(crate) X: FieldElement,
    pub(crate) Y: FieldElement,
    pub(crate) Z: FieldElement,
    pub(crate) T: FieldElement,
}

// ------------------------------------------------------------------------
// Constructors
// ------------------------------------------------------------------------

impl Default for EdwardsPoint {
    fn default() -> EdwardsPoint {
        EdwardsPoint {
            X: FieldElement::zero(),
            Y: FieldElement::one(),
            Z: FieldElement::one(),
            T: FieldElement::zero(),
        }
    }
}

// ------------------------------------------------------------------------
// Doubling
// ------------------------------------------------------------------------

impl EdwardsPoint {
    /// Add this point to itself.
    pub(crate) fn double(&self) -> EdwardsPoint {
        self + self
    }
}

// ------------------------------------------------------------------------
// Addition and Subtraction
// ------------------------------------------------------------------------

impl<'a, 'b> Add<&'b EdwardsPoint> for &'a EdwardsPoint {
    type Output = EdwardsPoint;
    fn add(self, other: &'b EdwardsPoint) -> EdwardsPoint {
        let A = self.X * other.X;
        let B = self.Y * other.Y;
        let C = EDWARDS_D * self.T * other.T;
        let D = self.Z * other.Z;
        let E = (self.X + self.Y) * (other.X + other.Y) - A - B;
        let F = D - C;
        let G = D + C;
        let H = B + A;

        EdwardsPoint {
            X: E * F,
            Y: G * H,
            Z: F * G,
            T: E * H,
        }
    }
}

impl<'a, 'b> Sub<&'b EdwardsPoint> for &'a EdwardsPoint {
    type Output = EdwardsPoint;
    fn sub(self, other: &'b EdwardsPoint) -> EdwardsPoint {
        self + &-other
    }
}

// ------------------------------------------------------------------------
// Negation
// ------------------------------------------------------------------------

impl<'a> Neg for &'a EdwardsPoint {
    type Output = EdwardsPoint;

    fn neg(self) -> EdwardsPoint {
        EdwardsPoint {
            X: -self.X,
            Y: self.Y,
            Z: self.Z,
            T: -self.T,
        }
    }
}

impl Neg for EdwardsPoint {
    type Output = EdwardsPoint;

    fn neg(self) -> EdwardsPoint {
        -&self
    }
}
