extern crate num;

use num::integer::gcd;

fn main() {
    let mut tmp = 1;
    for i in 2..20 {
        tmp = tmp * i / gcd(tmp, i);
    }
    println!("{}", tmp);
}
