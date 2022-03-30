//! 找到和为给定整数的三个连续整数

pub fn sum_of_three(num: i64) -> Vec<i64> {
    if num % 3 != 0 { return vec![]; }
    let mid = num / 3;
    vec![mid - 1, mid, mid + 1]
}

fn main() {
    assert_eq!(sum_of_three(33), vec![10, 11, 12]);
    assert_eq!(sum_of_three(4), vec![]);
    assert_eq!(sum_of_three(0), vec![-1, 0, 1]);
}
