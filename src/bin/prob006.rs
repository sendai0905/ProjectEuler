pub fn main() {
    let mut n = 0;
    for i in 1..101 {
        n += i * i;
    }
    println!("{}", 5050 * 5050 - n);
}
