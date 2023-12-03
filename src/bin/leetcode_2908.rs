//! 元素和最小的山形三元组 I

pub fn minimum_sum(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut result = i32::MAX;
    for i in 0..len {
        for j in i + 1..len {
            for k in j + 1..len {
                if nums[i] < nums[j] && nums[j] > nums[k] {
                    result = result.min(nums[i] + nums[j] + nums[k]);
                }
            }
        }
    }
    if result == i32::MAX { -1 } else { result }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![8, 6, 1, 5, 3]), 9);
        assert_eq!(func(vec![5, 4, 8, 7, 10, 2]), 13);
        assert_eq!(func(vec![6, 5, 4, 3, 4, 5]), -1);
    }
    test(minimum_sum);
}
