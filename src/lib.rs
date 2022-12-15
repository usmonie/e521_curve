extern crate core;

use std::ops::Mul;
use std::str::FromStr;

use num_bigint_dig::{BigInt, RandBigInt};
use sha3::{Digest, Sha3_256};

use crate::e521::{Point, PointOperations};

mod e521;

const X: &str = "1571054894184995387535939749894317568645297350402905821437625181152304994381188529632591196067604100772673927915114267193389905003276673749012051148356041324";

pub fn generate_salt() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let private_key = rng.gen_bigint(128);

    private_key.to_bytes_be().1
}

pub fn generate_private_key() -> BigInt {
    let mut rng = rand::thread_rng();
    let private_key = rng.gen_bigint(1000);

    private_key
}

pub fn generate_public_key(private_key: &BigInt) -> Point {
    let e521 = Point::from(&BigInt::from_str(X).unwrap());
    e521.multiple_number_by_montgomery(private_key)
}

pub fn diffie_hellman(private_key: &BigInt, public_key: &Point) -> Point {
    public_key.multiple_number_by_montgomery(private_key)
}

pub fn generate_secret_key(point: Point) -> Vec<u8> {
    let key = point.x.mul(point.y);
    let key: Vec<u8> = key.to_bytes_be().1;
    return Sha3_256::digest(key).as_slice().to_vec();
}