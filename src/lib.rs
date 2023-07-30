extern crate core;
pub mod e521;

use std::ops::Mul;
use num_bigint::BigInt;
use num_bigint_dig::RandBigInt;

use sha3::{Digest, Sha3_256};
use crate::e521::e521::{get_e521_gen_point, Point, sec_mul};

const X: &str = "1571054894184995387535939749894317568645297350402905821437625181152304994381188529632591196067604100772673927915114267193389905003276673749012051148356041324";

pub fn generate_salt() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let private_key = rng.gen_bigint(128);

    private_key.to_bytes_be().1
}

pub fn generate_private_key() -> BigInt {
    let mut rng = rand::thread_rng();
    let private_key = rng.gen_bigint(1000);

    BigInt::from_signed_bytes_be(private_key.to_signed_bytes_be().as_slice())
}

pub fn generate_public_key(private_key: &BigInt) -> Point {
    let e521 = get_e521_gen_point(false);
    sec_mul(private_key, e521)
}

pub fn diffie_hellman(private_key: &BigInt, public_key: &Point) -> Point {
    sec_mul(private_key, public_key.clone())
}

pub fn generate_secret_key(point: Point) -> Vec<u8> {
    let key = point.x.mul(point.y);
    let key: Vec<u8> = key.to_bytes_be().1;
    return Sha3_256::digest(key).as_slice().to_vec();
}