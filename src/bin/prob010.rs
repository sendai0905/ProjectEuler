use euler::utils::sieve;

pub fn main() {
    let mut ans: i64 = 0;
    let prime_flags = sieve(2_000_000);
    for i in 2..2_000_000 {
        if prime_flags[i] {
            ans += i as i64;
        }
    }
    println!("{}", ans);
}
