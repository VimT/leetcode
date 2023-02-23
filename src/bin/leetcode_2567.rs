//! 修改两个元素的最小分数

pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
    if nums.len() <= 3 { return 0; }
    nums.sort_unstable();
    let len = nums.len();
    (nums[len - 1] - nums[2]).min(nums[len - 3] - nums[0]).min(nums[len - 2] - nums[1])
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 4, 3]), 0);
        assert_eq!(func(vec![1, 4, 7, 8, 5]), 3);
    }
    test(minimize_sum);
}
