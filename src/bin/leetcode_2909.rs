//! 元素和最小的山形三元组 II

pub fn minimum_sum(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut right_min = vec![0; len];
    let mut cur_min = i32::MAX;
    for i in (0..len).rev() {
        right_min[i] = cur_min;
        cur_min = cur_min.min(nums[i]);
    }
    let mut result = i32::MAX;
    cur_min = nums[0];
    for i in 1..len - 1 {
        if nums[i] > cur_min && nums[i] > right_min[i] {
            result = result.min(cur_min + nums[i] + right_min[i]);
        }
        cur_min = cur_min.min(nums[i]);
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
