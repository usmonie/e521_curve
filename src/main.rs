use std::str::FromStr;
use num_bigint::BigInt;
use crate::e521::e521::{get_e521_gen_point, get_e521_point_for_x};

pub mod e521;


const X: &str = "1571054894184995387535939749894317568645297350402905821437625181152304994381188529632591196067604100772673927915114267193389905003276673749012051148356041324";

fn main() {
    let point = get_e521_point_for_x(BigInt::from_str(X).unwrap(), false);
    println!("point y = {}", point.y.to_string());

}