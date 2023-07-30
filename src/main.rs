mod e521;

use std::str::FromStr;
use crate::e521::e521::get_e521_gen_point;

const X: &str = "1571054894184995387535939749894317568645297350402905821437625181152304994381188529632591196067604100772673927915114267193389905003276673749012051148356041324";

fn main() {
    let point = get_e521_gen_point(false);

    println!("point x = {:?}, y = {:?}", point.x.to_string(), point.y.to_string());
}