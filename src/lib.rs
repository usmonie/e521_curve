extern crate core;

use std::ops::Mul;

use rand::RngCore;
use rand::rngs::OsRng;
use rug::{Complete, Integer};
use rug::integer::Order;
use rug::rand::RandState;
use sha3::{Digest, Sha3_256};

use crate::e521::{MultiplePointMontgomery, PointE521};

pub mod e521;

const X: &str = "1571054894184995387535939749894317568645297350402905821437625181152304994381188529632591196067604100772673927915114267193389905003276673749012051148356041324";

pub fn generate_salt() -> Vec<u8> {
    let rng = OsRng.next_u32();
    let seed = Integer::from(rng);

    let mut rand = RandState::new();
    rand.seed(&seed);
    let salt: Vec<u8> = Integer::from(Integer::random_bits(128, &mut rand)).to_digits(Order::MsfBe);

    salt
}

pub fn generate_private_key() -> Integer {
    let rng = OsRng.next_u32();
    let seed = Integer::from(rng);

    let mut rand = RandState::new();
    rand.seed(&seed);
    let private_key = Integer::from(Integer::random_bits(512, &mut rand));

    private_key
}

pub fn generate_public_key(private_key: &Integer) -> PointE521 {
    let e521 = PointE521::create_from_x(&Integer::parse(X).unwrap().complete());

    e521.multiple_number_by_montgomery(private_key)
}

pub fn diffie_hellman(private_key: &Integer, public_key: &PointE521) -> PointE521 {
    public_key.multiple_number_by_montgomery(private_key)
}

pub fn generate_secret_key(point: PointE521) -> Vec<u8> {
    let key = point.x.mul(point.y);
    let key: Vec<u8> = key.to_digits(Order::MsfBe);
    return Sha3_256::digest(key).as_slice().to_vec();
}