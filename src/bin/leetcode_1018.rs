//! 可被 5 整除的二进制前缀

pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut num = 0;
    let mut result = Vec::with_capacity(nums.len());
    for v in nums {
        num = (num * 2 + v) % 5;
        result.push(num == 0);
    }
    result
}

pub fn prefixes_div_by5_another(nums: Vec<i32>) -> Vec<bool> {
    nums.iter().scan(0, |acc, &x| {
        *acc = (*acc * 2 + x) % 5;
        Some(*acc == 0)
    }).collect()
}

fn main() {
    assert_eq!(prefixes_div_by5(vec![0, 1, 1]), vec![true, false, false]);
    assert_eq!(prefixes_div_by5(vec![1, 1, 1]), vec![false, false, false]);
}
