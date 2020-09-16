fn count_string(s: &str) -> i32 {
    s.split_whitespace().map(|n| n.len() as i32).sum()
}

fn count_from_1_9() -> i32 {
    count_string("one two three four five six seven eight nine")
}

fn count_from_10_19() -> i32 {
    count_string("ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen")
}

fn count_from_20_90() -> i32 {
    count_string("twenty thirty forty fifty sixty seventy eighty ninety")
}

fn count_from_1_99() -> i32 {
    count_from_1_9() * 9 + count_from_10_19() + count_from_20_90() * 10
}

fn count_from_100_999() -> i32 {
    (count_from_1_99() + "hundred".len() as i32 * 100 + "and".len() as i32 * 99) * 9
        + count_from_1_9() * 100
}

fn main() {
    let ans =
        count_from_1_99() + count_from_100_999() + "thousand".len() as i32 + "one".len() as i32;
    println!("{}", ans);
}
