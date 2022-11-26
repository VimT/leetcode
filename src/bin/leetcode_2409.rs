//! 统计共同度过的日子数

fn date2day(date: String) -> i32 {
    static M: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let (m, d) = date.split_once("-").unwrap();
    let m: i32 = m.parse().unwrap();
    let d: i32 = d.parse().unwrap();
    M[..m as usize - 1].iter().sum::<i32>() + d
}

pub fn count_days_together(arrive_alice: String, leave_alice: String, arrive_bob: String, leave_bob: String) -> i32 {
    let a = date2day(arrive_alice);
    let b = date2day(leave_alice);
    let c = date2day(arrive_bob);
    let d = date2day(leave_bob);
    if a <= d && c <= b {
        // 区间相交长度
        return b.min(d) - a.max(c) + 1;
    }
    0
}

fn main() {
    assert_eq!(count_days_together(String::from("09-01"), String::from("10-19"), String::from("06-19"), String::from("10-20")), 49);
    assert_eq!(count_days_together(String::from("10-01"), String::from("10-31"), String::from("11-01"), String::from("12-31")), 0);
    assert_eq!(count_days_together(String::from("08-15"), String::from("08-18"), String::from("08-16"), String::from("08-19")), 3);
}
