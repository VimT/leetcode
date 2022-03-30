//! 只出现一次的数字 III

pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mut tmp = 0;
    for &i in &nums {
        tmp ^= i;
    }
    let mut iota = 1;
    while iota & tmp == 0 {
        iota <<= 1;
    }
    let mut a = 0;
    let mut b = 0;
    for i in nums {
        if i & iota > 0 {
            a ^= i;
        } else {
            b ^= i;
        }
    }
    vec![a, b]
}

fn main() {
    assert_eq!(single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
    assert_eq!(single_number(vec![-1, 0]), vec![-1, 0]);
    assert_eq!(single_number(vec![0, 1]), vec![1, 0]);
}
