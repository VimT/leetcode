//! 坏了的计算器

pub fn broken_calc(start_value: i32, mut target: i32) -> i32 {
    let mut result = 0;
    while start_value < target {
        if target & 1 == 0 {
            target >>= 1;
            result += 1;
        } else {
            target = (target + 1) >> 1;
            result += 2;
        }
    }
    result + start_value - target
}

fn main() {
    assert_eq!(broken_calc(2, 3), 2);
    assert_eq!(broken_calc(5, 8), 2);
    assert_eq!(broken_calc(3, 10), 3);
}
