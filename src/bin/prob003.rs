pub fn main() {
    let mut n: i64 = 600851475143;
    let mut ans = 1;
    for i in 2..(n as f64).sqrt().floor() as i64 {
        if n % i == 0 {
            ans = i;
            n /= i;
        }
    }
    println!("{}", ans);
}
