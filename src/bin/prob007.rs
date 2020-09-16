use euler::utils::is_prime;

fn main() {
    let mut ans = 1;
    let mut count = 0;
    while count != 10001 {
        ans += 1;
        if is_prime(ans) {
            count += 1;
        }
    }
    println!("{}", ans);
}
