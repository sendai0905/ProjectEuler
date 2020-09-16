extern crate num;

use num::bigint::ToBigUint;
use num::BigUint;

pub fn main() {
    let mut tmp: BigUint = 1.to_biguint().unwrap();
    for _ in 0..1000 {
        tmp *= 2.to_biguint().unwrap();
    }
    let mut ans = 0;
    let tmps = tmp.to_string();
    for i in tmps.chars() {
        ans += i as i32 - 48;
    }

    println!("{}", ans);
}
