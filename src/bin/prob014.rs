fn main() {
    let (mut ans, mut max_cnt) = (0, 0);
    for i in 1..1_000_001 {
        let mut cnt = 0;
        let mut tmp: i64 = i;
        while tmp != 1 {
            if tmp % 2 == 1 {
                tmp += tmp * 2 + 1;
            } else {
                tmp /= 2;
            }
            cnt += 1;
        }
        if max_cnt < cnt {
            max_cnt = cnt;
            ans = i;
        }
    }
    println!("{}", ans);
}
