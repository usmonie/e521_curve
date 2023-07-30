use std::fmt;
use std::fmt::Display;
use std::ops::{BitAnd, Shl, Shr, Sub};
use std::str::FromStr;

use num::{One, Signed, ToPrimitive, Zero};
use num::pow::Pow;
use num_bigint_dig::{BigInt, ModInverse};

const P: &str = "6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057151";
const D: i64 = -376014;

pub struct Point {
    pub x: BigInt,
    pub y: BigInt,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:?}, y: {:?}", self.x.to_string(), self.y.to_string())
    }
}

impl Clone for Point {
    #[inline]
    fn clone(&self) -> Self {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.x = other.x.clone();
        self.y.clone_from(&other.y.clone());
    }
}

pub trait PointOperations {
    fn multiple_number_by_montgomery(&self, n: &BigInt) -> Point;

    fn add(&self, point: &Point) -> Point;
}

impl Point {
    pub fn from(x: &BigInt) -> Self {
        let one = BigInt::one();
        let p = BigInt::from_str(P).unwrap();
        let d = BigInt::from(D);

        let x_powed: BigInt = x.pow(2 as u64);
        let num: BigInt = &one - &x_powed % &p;

        let d_mul = d * x_powed % &p;
        let denom = one - d_mul;
        let denom = denom.mod_inverse(&p).unwrap();

        let radicand = num * denom;

        Point {
            x: x.clone(),
            y: sqrt(&radicand, &p, true).unwrap(),
        }
    }

    fn get_new_x(one: &BigInt, p: &BigInt, d: &BigInt, x1: &BigInt, x2: &BigInt, y1: &BigInt, y2: &BigInt) -> BigInt {
        let x_num = (x1 * y2 + y1 * x2) % p;
        let x_denom = (one + d * x1 * x2 * y1 * y2) % p;
        let x_denom = x_denom.mod_inverse(p).unwrap();

        (x_num * x_denom) % p
    }

    fn get_new_y(one: &BigInt, p: &BigInt, d: &BigInt, x1: &BigInt, x2: &BigInt, y1: &BigInt, y2: &BigInt) -> BigInt {
        let y_num = (y1 * y2 - x1 * x2) % p;
        let y_denom = (one - d * x1 * x2 * y1 * y2) % p;
        let y_denom = y_denom.mod_inverse(p).unwrap();
        y_num * y_denom % p
    }
}

impl PointOperations for Point {
    fn multiple_number_by_montgomery(&self, n: &BigInt) -> Point {
        let mut r0 = Point {
            x: BigInt::zero(),
            y: BigInt::one(),
        };

        let mut r1 = self.clone();
        let mut idx = n.to_bytes_be().1.len().to_isize().unwrap();

        while idx >= 0 {
            if n.get_bit_at(idx) {
                r0 = r0.add(&r1);
                r1 = r1.add(&r1);
            } else {
                r1 = r0.add(&r1);
                r0 = r0.add(&r0);
            }
            idx -= 1;
        }
        r0
    }

    fn add(&self, point: &Point) -> Point {
        let one = BigInt::one();
        let p = BigInt::from_str(P).unwrap();
        let d = BigInt::from(D);

        let x1 = &self.x;
        let x2 = &point.x;

        let y1 = &self.y;
        let y2 = &point.y;

        let x = Self::get_new_x(&one, &p, &d, x1, x2, y1, y2);

        let y = Self::get_new_y(&one, &p, &d, x1, x2, y1, y2);

        Point {
            x,
            y,
        }
    }
}

impl Point {
    pub fn is_on_curve(&self) -> bool {
        let p = BigInt::from_str(P).unwrap();
        let d = BigInt::from(D);

        let x = &self.x;
        let y = &self.y;

        let x_squared = x.pow(2 as u64) % &p;
        let y_squared = y.pow(2 as u64) % &p;

        let left_side = (x_squared.clone() + y_squared.clone()) % &p;
        let right_side = (BigInt::one() + d * x_squared * y_squared) % &p;

        left_side == right_side
    }
}

fn sqrt(v: &BigInt, p: &BigInt, lsb: bool) -> Option<BigInt> {
    let four = &BigInt::from(4);
    let p_mod = p % four;
    assert_eq!(p_mod, BigInt::from(3));

    let zero = BigInt::zero();
    let one = BigInt::one();
    if v.signum() == zero {
        return Some(zero);
    }

    let shifted = p.clone().shr(2);
    let added = shifted + one;
    let mut r = &v.clone().modpow(&added, p);

    if r.get_bit_at(0) != lsb {
        let tmp = p.sub(r);

        r = &tmp;

        let expr = (r * r - v) % p;

        return if expr.signum() == zero {
            Some(r.clone())
        } else {
            None
        };
    }

    Some(r.clone())
}

trait BitAt {
    fn get_bit_at(&self, n: isize) -> bool;
}

impl BitAt for BigInt {
    fn get_bit_at(&self, n: isize) -> bool {
        self.bitand(BigInt::from(1).shl(n.to_usize().unwrap())) != BigInt::zero()
    }
}