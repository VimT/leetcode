//! 统计数组中相等且可以被整除的数对

pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut result = 0;
    let k = k as usize;
    for i in 0..len {
        for j in i + 1..len {
            if nums[i] == nums[j] && (i * j) % k == 0 {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
    assert_eq!(count_pairs(vec![1, 2, 3, 4], 1), 0);
}
