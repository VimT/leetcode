//! 转换数字的最少位翻转次数

pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
    (start ^ goal).count_ones() as i32
}

fn main() {
    assert_eq!(min_bit_flips(10, 7), 3);
    assert_eq!(min_bit_flips(3, 4), 3);
}
