//! 统计各位数字之和为偶数的整数个数

pub fn count_even(num: i32) -> i32 {
    let mut result = 0;
    for mut i in 2..=num {
        let mut sum = 0;
        while i > 0 {
            sum += i % 10;
            i /= 10;
        }
        if sum & 1 == 0 { result += 1; }
    }
    result
}

fn main() {
    assert_eq!(count_even(4), 2);
    assert_eq!(count_even(30), 14);
}
