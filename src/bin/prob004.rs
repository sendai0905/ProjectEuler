use std::cmp::max;

fn check_palindromic_number(n: i32) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let mut ans = 0;
    for i in 100..999 {
        for j in 100..999 {
            if check_palindromic_number(i * j) {
                ans = max(ans, i * j);
            }
        }
    }
    println!("{}", ans);
}
