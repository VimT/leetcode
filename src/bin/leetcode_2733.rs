//! 既不是最小值也不是最大值

pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
    let mut mn = nums[0];
    let mut mx = nums[0];
    for &num in &nums {
        mn = mn.min(num);
        mx = mx.max(num);
    }
    for num in nums {
        if num != mn && num != mx { return num; }
    }
    -1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 2, 1, 4]), 3);
        assert_eq!(func(vec![1, 2]), -1);
        assert_eq!(func(vec![2, 1, 3]), 2);
    }
    test(find_non_min_or_max);
}
