//! 最大连续 1 的个数

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max_count = 0;
    let mut count = 0;
    let len = nums.len();
    for i in 0..len {
        if nums[i] == 1 {
            count += 1;
        } else {
            max_count = max_count.max(count);
            count = 0;
        }
    }
    max_count.max(count)
}

fn main() {
    assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
}
