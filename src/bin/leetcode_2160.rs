//! 拆分数位后四位数字的最小和

pub fn minimum_sum(num: i32) -> i32 {
    let mut result = i32::MAX;
    let mut n = num;
    let mut s = vec![];
    while n > 0 {
        s.push(n % 10);
        n /= 10;
    }
    s.sort_unstable();
    result = result.min(s[3] + s[0] * 100 + s[1] * 10 + s[2]);
    result = result.min(s[0] * 10 + s[1] * 10 + s[2] + s[3]);
    result
}

fn main() {
    assert_eq!(minimum_sum(2932), 52);
    assert_eq!(minimum_sum(4009), 13);
}
