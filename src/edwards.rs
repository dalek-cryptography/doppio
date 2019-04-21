//! Point arithmetic for the doppio curve.
//!
//! This implements the equations for point arithmetic from
//! HWCD: https://eprint.iacr.org/2008/522.pdf

#![allow(non_snake_case)]

use core::ops::{Add, Neg, Sub};

use crate::field::{FieldElement, EDWARDS_D};
use crate::Ristretto255Scalar;

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
    // TODO: add a test that A.double() = A * A
    fn double(&self) -> EdwardsPoint {
        let two: FieldElement = Ristretto255Scalar::from(2u8).into();

        let A = self.X * self.X;
        let B = self.Y * self.Y;
        // Is it cheaper to add Z*Z + Z*Z or multiply 2*Z*Z?
        let C = two * self.Z * self.Z;
        // a = -1
        let D = -A;
        let E = (self.X + self.Y) * (self.X + self.Y) - A - B;
        let G = D + B;
        let F = G - C;
        let H = D - B;

        EdwardsPoint {
            X: E * F,
            Y: G * H,
            Z: F * G,
            T: E * H,
        }
    }
}

// ------------------------------------------------------------------------
// Addition and Subtraction
// ------------------------------------------------------------------------

impl<'a, 'b> Add<&'b EdwardsPoint> for &'a EdwardsPoint {
    type Output = EdwardsPoint;

    fn add(self, other: &'b EdwardsPoint) -> EdwardsPoint {
        // k = 2d'. d' = -d/a and a = -1, so k = 2d.
        let two: FieldElement = Ristretto255Scalar::from(2u8).into();
        let k = EDWARDS_D * two;

        let A = (self.Y - self.X) * (other.Y - other.X);
        let B = (self.Y + self.X) * (other.Y + other.X);
        let C = k * self.T * other.T;
        let D = two * self.Z * other.Z;
        let E = B - A;
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

    // TODO: add a test that A - B = A + -B
    fn sub(self, other: &'b EdwardsPoint) -> EdwardsPoint {
        // The same equation as addition, except other.X and other.T are negated.
        // k = 2d'. d' = -d/a and a = -1, so k = 2d.
        let two = Ristretto255Scalar::from(2u8).into();
        let k = EDWARDS_D * two;

        let A = (self.Y - self.X) * (other.Y + other.X);
        let B = (self.Y + self.X) * (other.Y - other.X);
        let C = k * self.T * other.T;
        let D = two * self.Z * other.Z;
        let E = B - A;
        let F = D + C;
        let G = D - C;
        let H = B + A;

        EdwardsPoint {
            X: E * F,
            Y: G * H,
            Z: F * G,
            T: E * H,
        }
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
