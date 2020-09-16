use euler::utils::make_divisors;

fn main() {
    let mut ans: i64 = 0;
    let mut d = 1;
    loop {
        ans += d;
        d += 1;
        let div = make_divisors(ans);
        if div.len() > 500 {
            break;
        }
    }
    println!("{}", ans);
}
