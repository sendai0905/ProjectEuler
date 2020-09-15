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
