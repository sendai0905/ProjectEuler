use std::vec::Vec;

pub fn is_prime(n: i32) -> bool {
    if n == 1 {
        false
    } else {
        let lim = (n as f64).sqrt().floor() as i32 + 1;
        for i in 2..lim {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

pub fn sieve(n: usize) -> Vec<bool> {
    let mut flags = vec![true; n];
    flags[0] = false;
    flags[1] = false;
    for i in (4..n).step_by(2) {
        flags[i] = false;
    }
    for i in (3..n).step_by(2) {
        if flags[i] {
            for j in (i * i..n).step_by(i) {
                flags[j] = false;
            }
        }
    }
    flags
}
