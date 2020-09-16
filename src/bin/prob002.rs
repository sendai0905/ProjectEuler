fn main() {
    let mut fib = vec![1, 2];
    let mut ans = 2;
    while fib.last() < Some(&4000000) {
        let l = fib.len();
        let n = fib[l - 1] + fib[l - 2];
        fib.push(n);
        if n % 2 == 0 {
            ans += n;
        }
    }
    println!("{}", ans);
}
