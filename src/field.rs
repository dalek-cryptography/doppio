//! Field arithmetic for the doppio curve.
//!
//! Because doppio is intended for use in ristretto255-based proof
//! systems, the scalar field of ristretto255 is the *ground field*
//! for doppio.
//!
//! This implementation is derived from the 52-bit scalar
//! implementation contributed to `curve25519-dalek` by Andrew Moon.

use std::default::Default;
use std::ops::{Add, Mul, Sub};

use crate::Ristretto255Scalar;

/// A field element modulo \\(2\^{252} +
/// 27742317777372353535851937790883648493\\), the ground field for
/// the doppio curve and the scalar field for the ristretto255 group.
#[derive(Copy, Clone, Debug)]
pub struct FieldElement([u64; 5]);

const L: FieldElement = FieldElement([
    0x0002631a5cf5d3ed,
    0x000dea2f79cd6581,
    0x000000000014def9,
    0x0000000000000000,
    0x0000100000000000,
]);
const LFACTOR: u64 = 0x51da312547e1b;

/// `R` = R % L where R = 2^260
const R: FieldElement = FieldElement([
    0x000f48bd6721e6ed,
    0x0003bab5ac67e45a,
    0x000fffffeb35e51b,
    0x000fffffffffffff,
    0x00000fffffffffff,
]);

/// `RR` = (R^2) % L where R = 2^260
const RR: FieldElement = FieldElement([
    0x0009d265e952d13b,
    0x000d63c715bea69f,
    0x0005be65cb687604,
    0x0003dceec73d217f,
    0x000009411b7c309a,
]);

/// u64 * u64 = u128 multiply helper
#[inline(always)]
fn m(x: u64, y: u64) -> u128 {
    (x as u128) * (y as u128)
}

impl Add<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: FieldElement) -> FieldElement {
        let mut sum = FieldElement::zero();
        let mask = (1u64 << 52) - 1;

        // a + b
        let mut carry: u64 = 0;
        for i in 0..5 {
            let total = self.0[i] + rhs.0[i] + carry;
            carry = total >> 52;
            sum.0[i] = total & mask;
        }

        // subtract l if the sum is >= l
        // ?: if sum < l then it gets handled nicely?
        sum - L
    }
}

impl Sub<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: FieldElement) -> FieldElement {
        // let mut difference = FieldElement::zero();
        // let mask = (1u64 << 52) - 1;

        // // a - b
        // let mut borrow: u64 = 0;
        // for i in 0..5 {
        //     let total = self.0[i].wrapping_sub(rhs.0[i] + borrow);
        //     borrow = total >> 63;
        //     difference.0[i] = total & mask;
        // }

        // // conditionally add l if the difference is negative
        // unimplemented!()

        let mut difference = FieldElement::zero();
        let mask = (1u64 << 52) - 1;

        // a - b
        let mut borrow: u64 = 0;
        for i in 0..5 {
            borrow = self.0[i].wrapping_sub(rhs.0[i] + (borrow >> 63));
            difference.0[i] = borrow & mask;
        }

        // conditionally add l if the difference is negative
        let underflow_mask = ((borrow >> 63) ^ 1).wrapping_sub(1);
        let mut carry: u64 = 0;
        for i in 0..5 {
            carry = (carry >> 52) + difference.0[i] + (L.0[i] & underflow_mask);
            difference.0[i] = carry & mask;
        }

        difference
    }
}

impl Mul<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> FieldElement {
        FieldElement::montgomery_reduce(FieldElement::mul_internal(self, rhs))
    }
}

impl Default for FieldElement {
    fn default() -> FieldElement {
        FieldElement::zero()
    }
}

impl From<Ristretto255Scalar> for FieldElement {
    fn from(packed: Ristretto255Scalar) -> FieldElement {
        let bytes = packed.as_bytes();

        let mut words = [0u64; 4];
        for i in 0..4 {
            for j in 0..8 {
                words[i] |= (bytes[(i * 8) + j] as u64) << (j * 8);
            }
        }

        let mask = (1u64 << 52) - 1;
        let top_mask = (1u64 << 48) - 1;
        let mut s = FieldElement::zero();

        s.0[0] = words[0] & mask;
        s.0[1] = ((words[0] >> 52) | (words[1] << 12)) & mask;
        s.0[2] = ((words[1] >> 40) | (words[2] << 24)) & mask;
        s.0[3] = ((words[2] >> 28) | (words[3] << 36)) & mask;
        s.0[4] = (words[3] >> 16) & top_mask;

        s * RR
    }
}

impl Into<Ristretto255Scalar> for FieldElement {
    fn into(self) -> Ristretto255Scalar {
        let mut limbs = [0u128; 9];
        for i in 0..5 {
            limbs[i] = self.0[i] as u128;
        }

        let redc = FieldElement::montgomery_reduce(limbs);

        let mut s = [0u8; 32];

        s[0] = (redc.0[0] >> 0) as u8;
        s[1] = (redc.0[0] >> 8) as u8;
        s[2] = (redc.0[0] >> 16) as u8;
        s[3] = (redc.0[0] >> 24) as u8;
        s[4] = (redc.0[0] >> 32) as u8;
        s[5] = (redc.0[0] >> 40) as u8;
        s[6] = ((redc.0[0] >> 48) | (redc.0[1] << 4)) as u8;
        s[7] = (redc.0[1] >> 4) as u8;
        s[8] = (redc.0[1] >> 12) as u8;
        s[9] = (redc.0[1] >> 20) as u8;
        s[10] = (redc.0[1] >> 28) as u8;
        s[11] = (redc.0[1] >> 36) as u8;
        s[12] = (redc.0[1] >> 44) as u8;
        s[13] = (redc.0[2] >> 0) as u8;
        s[14] = (redc.0[2] >> 8) as u8;
        s[15] = (redc.0[2] >> 16) as u8;
        s[16] = (redc.0[2] >> 24) as u8;
        s[17] = (redc.0[2] >> 32) as u8;
        s[18] = (redc.0[2] >> 40) as u8;
        s[19] = ((redc.0[2] >> 48) | (redc.0[3] << 4)) as u8;
        s[20] = (redc.0[3] >> 4) as u8;
        s[21] = (redc.0[3] >> 12) as u8;
        s[22] = (redc.0[3] >> 20) as u8;
        s[23] = (redc.0[3] >> 28) as u8;
        s[24] = (redc.0[3] >> 36) as u8;
        s[25] = (redc.0[3] >> 44) as u8;
        s[26] = (redc.0[4] >> 0) as u8;
        s[27] = (redc.0[4] >> 8) as u8;
        s[28] = (redc.0[4] >> 16) as u8;
        s[29] = (redc.0[4] >> 24) as u8;
        s[30] = (redc.0[4] >> 32) as u8;
        s[31] = (redc.0[4] >> 40) as u8;

        Ristretto255Scalar::from_bits(s)
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

    #[inline]
    fn mul_internal(lhs: FieldElement, rhs: FieldElement) -> [u128; 9] {
        let mut z = [0u128; 9];
        let a = &lhs.0;
        let b = &rhs.0;

        z[0] = m(a[0], b[0]);
        z[1] = m(a[0], b[1]) + m(a[1], b[0]);
        z[2] = m(a[0], b[2]) + m(a[1], b[1]) + m(a[2], b[0]);
        z[3] = m(a[0], b[3]) + m(a[1], b[2]) + m(a[2], b[1]) + m(a[3], b[0]);
        z[4] = m(a[0], b[4]) + m(a[1], b[3]) + m(a[2], b[2]) + m(a[3], b[1]) + m(a[4], b[0]);
        z[5] = m(a[1], b[4]) + m(a[2], b[3]) + m(a[3], b[2]) + m(a[4], b[1]);
        z[6] = m(a[2], b[4]) + m(a[3], b[3]) + m(a[4], b[2]);
        z[7] = m(a[3], b[4]) + m(a[4], b[3]);
        z[8] = m(a[4], b[4]);

        z
    }

    #[inline]
    fn montgomery_reduce(limbs: [u128; 9]) -> FieldElement {
        #[inline(always)]
        fn part1(sum: u128) -> (u128, u64) {
            let p = (sum as u64).wrapping_mul(LFACTOR) & ((1u64 << 52) - 1);
            ((sum + m(p, L.0[0])) >> 52, p)
        }

        #[inline(always)]
        fn part2(sum: u128) -> (u128, u64) {
            let w = (sum as u64) & ((1u64 << 52) - 1);
            (sum >> 52, w)
        }

        // note: l3 is zero, so its multiplies can be skipped
        let l = &L.0;

        // the first half computes the Montgomery adjustment factor n, and begins adding n*l to make limbs divisible by R
        let (carry, n0) = part1(limbs[0]);
        let (carry, n1) = part1(carry + limbs[1] + m(n0, l[1]));
        let (carry, n2) = part1(carry + limbs[2] + m(n0, l[2]) + m(n1, l[1]));
        let (carry, n3) = part1(carry + limbs[3] + m(n1, l[2]) + m(n2, l[1]));
        let (carry, n4) = part1(carry + limbs[4] + m(n0, l[4]) + m(n2, l[2]) + m(n3, l[1]));

        // limbs is divisible by R now, so we can divide by R by simply storing the upper half as the result
        let (carry, r0) = part2(carry + limbs[5] + m(n1, l[4]) + m(n3, l[2]) + m(n4, l[1]));
        let (carry, r1) = part2(carry + limbs[6] + m(n2, l[4]) + m(n4, l[2]));
        let (carry, r2) = part2(carry + limbs[7] + m(n3, l[4]));
        let (carry, r3) = part2(carry + limbs[8] + m(n4, l[4]));
        let r4 = carry as u64;

        // result may be >= l, so attempt to subtract l
        FieldElement::sub(FieldElement([r0, r1, r2, r3, r4]), L)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sub() {
        let a = FieldElement([(1 << 52) - 3, 10, 0, 0, 0]);
        let b = FieldElement([(1 << 52) - 1, 20, 0, 0, 0]);
        a - b;
        panic!();
    }
}
