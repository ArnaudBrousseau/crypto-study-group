use std::fmt;
use std::ops;

use crate::element::Element;
use crate::util::buint;
use num_bigint::BigUint;
use num_bigint::ToBigInt;
use num_bigint::ToBigUint;
use thiserror::Error;
use util::bint;

// `Element` is meant to be part of this library's public interface,
// but the util and eea modules aren't. They're internal helpers.
mod eea;
pub mod element;
mod util;

#[derive(Error, Debug)]
pub enum ECCError {
    #[error("Cannot operate on two different curves")]
    CurveMismatch,
    #[error("Cannot create a Point with field elements on different fields")]
    PointCoordinateFieldMismatch,
    #[error("Cannot compute modulus for point at infinity")]
    NoModulusForPointAtInfinity,
    #[error("Cannot compute EEA because one or more operand is zero")]
    EEAOperandIsZero,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurveForm {
    Weierstrass,
    Montgomery,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Curve<'a> {
    name: &'a str,
    p: BigUint,
    a2: Element,
    a4: Element,
    a6: Element,
}

impl fmt::Display for Curve<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Curve {}", self.name)?;
        match self.form() {
            CurveForm::Montgomery => write!(f, " (Montgomery):")?,
            CurveForm::Weierstrass => write!(f, " (Weierstrass):")?,
            CurveForm::Unknown => write!(f, ":")?,
        }
        write!(f, " y^2 = x^3")?;
        if self.a2.value != buint(0) {
            write!(f, " + {}x^2", self.a2.value)?;
        }
        if self.a4.value != buint(0) {
            write!(f, " + {}x", self.a4.value)?;
        }
        if self.a6.value != buint(0) {
            write!(f, " + {}", self.a6.value)?;
        }
        write!(f, " (mod {})>", self.p)
    }
}

impl Curve<'_> {
    pub fn new<BUINT: ToBigUint, BINT: ToBigInt>(
        name: &str,
        a2: BINT,
        a4: BINT,
        a6: BINT,
        p: BUINT,
    ) -> Curve {
        let prime = p.to_biguint().unwrap();

        Curve {
            name,
            p: prime.clone(),
            a2: Element::new(a2, prime.clone()),
            a4: Element::new(a4, prime.clone()),
            a6: Element::new(a6, prime),
        }
    }

    fn form(&self) -> CurveForm {
        if self.a4 == self.zero() && self.a2 != self.zero() {
            CurveForm::Montgomery
        } else if self.a2 == self.zero() && self.a4 != self.zero() {
            CurveForm::Weierstrass
        } else {
            // Curve isn't a Montgomery or Weierstrass curve
            // ...but it's still a valid elliptic curve!
            CurveForm::Unknown
        }
    }

    pub fn discriminant(&self) -> BigUint {
        // See https://mathworld.wolfram.com/EllipticDiscriminant.html for this formula
        let b2 = self.element(4) * self.a2.clone();
        let b4 = self.element(2) * self.a4.clone();
        let b6 = self.element(4) * self.a6.clone();
        let b8 = b2.clone() * self.a6.clone() - self.a4.pow(2);

        let delta = -b8 * b2.pow(2) - self.element(8) * b4.pow(3) - self.element(27) * b6.pow(2)
            + self.element(9) * b2 * b4 * b6;

        delta.value
    }

    pub fn is_smooth(&self) -> bool {
        self.discriminant() != buint(0)
    }

    pub fn contains(&self, point: Point) -> bool {
        if point.is_at_infinity() {
            true
        } else {
            let x = point.x;
            let y = point.y;
            y.pow(2)
                == (x.pow(3) + self.a2.clone() * x.pow(2) + self.a4.clone() * x + self.a6.clone())
        }
    }

    pub fn element<T: ToBigInt>(&self, value: T) -> Element {
        Element::new(value, self.p.clone())
    }

    pub fn point<T: ToBigInt>(&self, x: T, y: T) -> Point {
        Point {
            x: Element::new(bint(x), self.p.clone()),
            y: Element::new(bint(y), self.p.clone()),
            curve: self,
        }
    }

    pub fn point_at_infinity(&self) -> Point {
        Point {
            x: Element::new(0, self.p.clone()),
            y: Element::new(0, self.p.clone()),
            curve: self,
        }
    }

    pub fn zero(&self) -> Element {
        Element::new(0, self.p.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point<'p> {
    x: Element,
    y: Element,
    curve: &'p Curve<'p>,
}

impl fmt::Display for Point<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_at_infinity() {
            write!(f, "<Point at Infinity>")
        } else {
            let x = self.x.clone();
            let y = self.y.clone();
            write!(
                f,
                "<Point on curve {}: x={}, y={}>",
                self.curve.name, x.value, y.value
            )
        }
    }
}

impl ops::Neg for Point<'_> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        if self.is_at_infinity() {
            self
        } else {
            Self {
                x: self.x,
                y: -self.y,
                curve: self.curve,
            }
        }
    }
}

impl ops::Add for Point<'_> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if other.curve != self.curve {
            panic!("{}", ECCError::CurveMismatch)
        }

        // O + X = X
        if self.is_at_infinity() && !other.is_at_infinity() {
            return other;
        }
        // X + O = X
        if !self.is_at_infinity() && other.is_at_infinity() {
            return self;
        }
        // O + O = O
        if self.is_at_infinity() && other.is_at_infinity() {
            return self.curve.point_at_infinity();
        }

        // We're now in the regular case: two normal points!
        let x = self.x.clone();
        let y = self.y.clone();
        let other_x = other.x.clone();
        let other_y = other.y.clone();
        let modulus = self.modulus().unwrap();

        // P + -P = O
        if x == other_x && (y.clone() + other_y.clone()).is_zero() {
            return self.curve.point_at_infinity();
        }

        let lambda = if self == other {
            // Compute Lambda for doubling
            (self.curve.element(3) * x.pow(2) + self.curve.clone().a4)
                / (Element::new(2, modulus) * y.clone())
        } else {
            // Standard formula, when the two points are distinct
            (other_y - y.clone()) / (other_x.clone() - x.clone())
        };

        let result_x = lambda.pow(2) - x.clone() - other_x;
        let result_y = lambda * (x - result_x.clone()) - y;

        // X + Y (both regular points)
        Point {
            x: result_x,
            y: result_y,
            curve: self.curve,
        }
    }
}

impl Point<'_> {
    pub fn is_at_infinity(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }

    pub fn modulus(&self) -> Result<BigUint, ECCError> {
        if self.is_at_infinity() {
            return Err(ECCError::NoModulusForPointAtInfinity);
        }
        let x = self.x.clone();
        let y = self.y.clone();

        if x.modulus != y.modulus {
            return Err(ECCError::PointCoordinateFieldMismatch);
        }
        Ok(x.modulus)
    }

    // Point multiplication, implemented with the double-and-add technique
    // (deviates from the OG toy_ecc implementation, which did this with non-adjacent representation)
    pub fn mul(&self, scalar: BigUint) -> Point {
        if scalar == buint(0) {
            return self.curve.point_at_infinity();
        }

        let mut result = self.curve.point_at_infinity();
        // Will be shifted right until 0
        let mut multiplier = scalar;
        // Will be set to p, 2p, 4p, 8p, etc...
        let mut doubled_point = self.clone();
        while multiplier.clone() != buint(0) {
            if multiplier.clone() % buint(2) == buint(1) {
                result = result + doubled_point.clone();
            }
            doubled_point = doubled_point.clone() + doubled_point.clone();
            multiplier >>= 1;
        }

        result
    }
}

#[cfg(test)]
mod test {

    use crate::util::{bint, buint};
    use crate::Curve;
    use num_bigint::BigUint;

    fn curve_secp256k1() -> Curve<'static> {
        let p_secp256 = bint(2).pow(256) - bint(2).pow(32) - bint(977);
        Curve::new("secp256k1", 0, 0, 7, p_secp256)
    }

    // Curve from Introduction to Mathematical Cryptography
    // Chapter 5, section 2 and 3
    // Y^2 =X^3+3X+8 on F13
    fn curve_trivial() -> Curve<'static> {
        Curve::new("trivial curve", 0, 3, 8, 13)
    }

    #[test]
    fn test_curve_display() {
        let montgomery = Curve::new("MyCurve", 44, 0, 46, 89);
        let weierstrass = Curve::new("MyCurve", 0, 5, 42, 89);
        let unknown = Curve::new("MyCurve", 5, 7, 42, 89);
        let both = Curve::new("MyCurve", 0, 0, 42, 89);

        assert_eq!(
            format!("{montgomery}"),
            "<Curve MyCurve (Montgomery): y^2 = x^3 + 44x^2 + 46 (mod 89)>"
        );
        assert_eq!(
            format!("{weierstrass}"),
            "<Curve MyCurve (Weierstrass): y^2 = x^3 + 5x + 42 (mod 89)>"
        );
        assert_eq!(
            format!("{unknown}"),
            "<Curve MyCurve: y^2 = x^3 + 5x^2 + 7x + 42 (mod 89)>"
        );
        assert_eq!(
            format!("{both}"),
            "<Curve MyCurve: y^2 = x^3 + 42 (mod 89)>"
        );
    }

    #[test]
    fn test_point_display() {
        let curve = curve_secp256k1();
        let p = curve.point(buint(1), buint(2));

        let formatted = format!("{p}");
        assert_eq!(formatted, "<Point on curve secp256k1: x=1, y=2>");
    }

    #[test]
    fn test_curve_discriminant() {
        let c = curve_trivial();
        assert_eq!(true, c.is_smooth());
        assert_eq!(buint(4), c.discriminant());

        // Example from https://mathworld.wolfram.com/EllipticDiscriminant.html
        let cusp = Curve::new("Cusp", 0, 0, 0, 13);
        assert_eq!(buint(0), cusp.discriminant());
        assert_eq!(false, cusp.is_smooth());

        let node = Curve::new("Node", 0, -3, 2, 13);
        assert_eq!(buint(0), node.discriminant());
        assert_eq!(false, node.is_smooth());

        let secp = curve_secp256k1();
        // See https://neuromancer.sk/std/secg/secp256k1
        let secp256k1_discriminant = BigUint::parse_bytes(
            b"115792089237316195423570985008687907853269984665640564039457584007908834650495",
            10,
        )
        .unwrap();
        assert_eq!(true, secp.is_smooth());
        assert_eq!(secp256k1_discriminant, secp.discriminant());
    }

    #[test]
    fn test_negate_point() {
        let curve = Curve::new("My curve", 7, 0, 1, 17);
        let p = curve.point(1, 2);

        let minus_p = -p;

        assert_eq!(minus_p.x.value, buint(1));
        assert_eq!(minus_p.y.value, buint(15));
    }

    #[test]
    fn test_contains_point_at_infinity() {
        let c = curve_trivial();
        let inf = c.point_at_infinity();
        assert_eq!(true, c.contains(inf));
    }

    #[test]
    fn test_contains_regular_points() {
        let c = curve_trivial();
        let p = c.point(2, 3);

        let c = curve_trivial();
        assert_eq!(true, c.contains(p));

        let q = c.point(2, 8);
        assert_eq!(false, c.contains(q));
    }

    #[test]
    // Test cases taken from page 289 of ItMC book (addition table)
    fn test_point_addition_on_trivial_curve() {
        let c = curve_trivial();
        let p = c.point(2, 3);
        let q = c.point(9, 6);

        let sum = p.clone() + q;
        assert_eq!(sum.x.value, buint(12));
        assert_eq!(sum.y.value, buint(2));

        let double = p.clone() + p;
        assert_eq!(double.x.value, buint(12));
        assert_eq!(double.y.value, buint(11));
    }

    #[test]
    // Test case taken from page 289 of ItMC book (addition table)
    fn test_addition_yields_infinity() {
        let c = curve_trivial();
        let p = c.point(1, 8);
        let q = c.point(1, 5);

        let sum = p + q;
        println!("{}", sum);
        assert_eq!(sum.is_at_infinity(), true);
    }

    #[test]
    fn test_point_addition_on_secp256k1() {
        let x = BigUint::parse_bytes(
            b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16,
        )
        .unwrap();
        let y = BigUint::parse_bytes(
            b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            16,
        )
        .unwrap();
        let curve = curve_secp256k1();
        let g = curve.point(x, y);

        let sum = g.clone() + g.clone();

        // These test values were obtained by referencing another secp256k1 / Bitcoin library: my own, written in JS a while back, specifically for Bitcoin.
        // See https://github.com/ArnaudBrousseau/arnaudbrousseau.com/blob/master/static/labs/keys.deconstructed/keys.deconstructed.js
        let expected_x = BigUint::parse_bytes(
            b"C6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE5",
            16,
        )
        .unwrap();
        let expected_y = BigUint::parse_bytes(
            b"1AE168FEA63DC339A3C58419466CEAEEF7F632653266D0E1236431A950CFE52A",
            16,
        )
        .unwrap();

        // ----/!\ DIGRESSION /!\------------------------------------------------------------------------------------------------------------------------------
        // Another cool proof that these expected values are correct: let's derive the Bitcoin address for it!
        // These expected coordinates are for "2*G", which means our "secret" seed is simply "2".
        // To derive a new (uncompressed) Bitcoin address from (x, y) coordinates, we can compute:
        //   hash: ripemd160(sha256(04||X_COORD_IN_HEX||Y_COORD_IN_HEX))
        //   checksum: sha256(sha256(00||hash)) -- take the first 4 bytes
        //   address: base58encode(00||hash||checksum)
        //   (in the above, "||" denotes concatenation)
        //
        // ----
        //
        // On the CLI we can compute the first steps directly with `openssl`:
        //
        //   # Computes "hash"
        //   $ echo -n '04C6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE51AE168FEA63DC339A3C58419466CEAEEF7F632653266D0E1236431A950CFE52A' \
        //     | xxd -r -p \
        //     | openssl dgst -sha256 -binary \
        //     | openssl dgst -ripemd160
        //   d6c8e828c1eca1bba065e1b83e1dc2a36e387a42
        //
        //   # Now, checksum
        //   $ echo -n '00d6c8e828c1eca1bba065e1b83e1dc2a36e387a42' | xxd -r -p | openssl dgst -sha256 -binary | openssl dgst -sha256
        //   74d4b4aafae3b18583e99da13ff0dae6276a888f5dbd3a05eea50806ebda06c1
        //
        // Finally, we go to http://lenschulwitz.com/base58 to base58 encode `00d6c8e828c1eca1bba065e1b83e1dc2a36e387a4274d4b4aa`
        // ...and we find that the result is `1LagHJk2FyCV2VzrNHVqg3gYG4TSYwDV4m`
        // ...low and behold, that's also the uncompressed address indicated on https://keys.lol/bitcoin/1 on the second row (seed 2!)
        //
        // Hence proving the correctness of these expected values: we just showed that they yield the right BTC address!
        // --------------------------------------------------------------------------------------------------------------------------------------------------

        assert_eq!(sum.clone().x.value, expected_x);
        assert_eq!(sum.clone().y.value, expected_y);
        assert_eq!(sum.modulus().unwrap(), curve_secp256k1().p);
    }

    #[test]
    fn test_addition_with_infinity_point() {
        let c = curve_secp256k1();
        let p = c.point(1, 2);
        let inf = c.point(0, 0);

        assert_eq!(p.clone() + inf.clone(), p.clone());
        assert_eq!(inf.clone() + p.clone(), p.clone());
        assert_eq!(inf.clone() + inf.clone(), c.point_at_infinity());
        assert_eq!((p.clone() + -p.clone()), c.point_at_infinity());
    }

    #[test]
    fn test_point_multiplication() {
        let c = curve_trivial();
        let p = c.point(1, 8);

        let noop_mul = p.mul(buint(1));
        assert_eq!(noop_mul.x, p.x);
        assert_eq!(noop_mul.y, p.y);

        let double = p.mul(buint(2));
        assert_eq!(double.x.value, buint(2));
        assert_eq!(double.y.value, buint(3));

        let triple = p.mul(buint(3));
        assert_eq!(triple.x.value, buint(9));
        assert_eq!(triple.y.value, buint(6));

        let quadruple = p.mul(buint(4));
        assert_eq!(quadruple.x.value, buint(12));
        assert_eq!(quadruple.y.value, buint(11));

        let quintuple = p.mul(buint(5));
        assert_eq!(quintuple.x.value, buint(12));
        assert_eq!(quintuple.y.value, buint(2));

        let times_nine = p.mul(buint(9));
        assert_eq!(times_nine, c.point_at_infinity());

        // Any multiple of 9 should yield the point at infinity
        let times_thousand = p.mul(buint(9000));
        assert_eq!(times_thousand, c.point_at_infinity());
    }
}
