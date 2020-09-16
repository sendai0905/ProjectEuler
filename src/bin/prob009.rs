fn main() {
    'outer: for i in 1..334 {
        let lim = ((1000 - i) as f32 / 2.0).round() as i32;
        for j in i + 1..lim {
            let k = 1000 - i - j;
            if i * i + j * j == k * k {
                println!("a={}, b={}, c={}", i, j, k);
                println!("{}", i * j * k);
                break 'outer;
            }
        }
    }
}
