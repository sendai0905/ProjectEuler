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

pub fn make_divisors(n: i64) -> Vec<i64> {
    let mut div: Vec<i64> = Vec::new();
    for i in 1..(n as f64).sqrt().floor() as i64 {
        if n % i == 0 {
            div.push(i);
            div.push(n / i);
        }
    }
    div
}

// from https://qiita.com/osanshouo/items/869bf08e979831ebb662#knuth-%E3%81%AE%E6%96%B9%E6%B3%95
pub fn binom_asc(n: i64, k: i64) -> i64 {
    if k == 0 || k == n {
        1
    } else {
        binom_asc(n - 1, k - 1) * n / k
    }
}
