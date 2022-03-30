//! 使数组唯一的最小增量

pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut result = 0;
    let len = nums.len();
    for i in 1..len {
        if nums[i] <= nums[i - 1] {
            let add = nums[i - 1] - nums[i] + 1;
            nums[i] += add;
            result += add;
        }
    }
    result
}

fn main() {
    assert_eq!(min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]), 6);
    assert_eq!(min_increment_for_unique(vec![1, 2, 2]), 1);
}
