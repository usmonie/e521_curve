use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Mul, Shr, Sub};
use std::str::FromStr;

use num::ToPrimitive;
use rug::{Complete, Integer};
use rug::integer::Order;
use rug::ops::Pow;

const P: &str = "6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057151";
const D: i64 = -376014;

pub trait MultiplePointMontgomery {
    fn multiple_number_by_montgomery(&self, s: &Integer) -> PointE521;
}

pub trait AddPoint {
    fn add(&self, point: &PointE521) -> PointE521;
}

pub struct PointE521 {
    pub x: Integer,
    pub y: Integer,
}

impl Display for PointE521 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:?}, y: {:?}", self.x, self.y)
    }
}

impl PointE521 {
    pub fn create_from_x(x: &Integer) -> Self {
        let one = &Integer::from(1);
        let p = &Integer::from_str(P).unwrap();
        let d = Integer::from(D);

        let num = (one.sub(x.pow(2).complete())).pow_mod(one, p).unwrap();

        let mut denom = (one.sub(d.mul(x.pow(2).complete()))).pow_mod(one, p).unwrap();
        denom = denom.invert(p).unwrap();
        let radicand = num.mul(denom);

        PointE521 {
            x: x.clone(),
            y: sqrt(&radicand, p, true).unwrap(),
        }
    }
}

impl Clone for PointE521 {
    #[inline]
    fn clone(&self) -> Self {
        PointE521 {
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

impl MultiplePointMontgomery for PointE521 {
    fn multiple_number_by_montgomery(&self, s: &Integer) -> PointE521 {
        let mut r0 = PointE521 {
            x: Integer::from(0),
            y: Integer::from(1),
        };

        let mut r1 = self.clone();
        let mut idx = s.to_digits::<u8>(Order::MsfBe).len().to_i64().unwrap() - 1;

        while idx >= 0 {
            if s.get_bit(idx.to_u32().unwrap()) {
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
}

impl AddPoint for PointE521 {
    fn add(&self, point: &PointE521) -> PointE521 {
        let p = Integer::from_str(P).unwrap();
        let one = &Integer::from(1);
        let d = Integer::from(D);

        let x1 = &self.x;
        let x2 = &point.x;

        let y1 = &self.y;
        let y2 = &point.y;

        let x_num = (x1.mul(y2).complete().add(y1.mul(x2))).pow_mod(one, &p).unwrap();
        let mut x_denom = (one.add(&D.mul(x1).complete().mul(x2).mul(y1).mul(y2))).complete().pow_mod(one, &p).unwrap();
        x_denom = x_denom.invert(&p).unwrap();

        let new_x = x_num.mul(x_denom).pow_mod(one, &p).unwrap();

        let y_num = (y1.mul(y2).complete().sub(x1.mul(x2))).pow_mod(one, &p).unwrap();
        let mut y_denom = (one.sub(d.mul(x1).mul(x2).mul(y1).mul(y2))).pow_mod(one, &p).unwrap();

        y_denom = y_denom.invert(&p).unwrap();
        let new_y = y_num.mul(y_denom).pow_mod(one, &p).unwrap();

        PointE521 {
            x: new_x,
            y: new_y,
        }
    }
}

fn sqrt(v: &Integer, p: &Integer, lsb: bool) -> Option<Integer> {
    assert!(p.get_bit(0) && p.get_bit(1));

    let zero = Integer::new();
    let one = &Integer::from(1);

    if v.clone().signum() == zero {
        return Some(zero);
    }

    let shifted: Integer = p.clone().shr(2);
    let added = shifted.add(one);
    let mut r = &v.clone().pow_mod(&added, p).unwrap();

    if r.get_bit(0) != lsb {
        let tmp = p.sub(r).complete();
        r = &tmp;
        let expr = (r.mul(r).sub(v).complete().pow_mod(one, p)).unwrap();

        return if expr.signum() == zero {
            Some(r.clone())
        } else {
            None
        };
    }

    Some(r.clone())
}