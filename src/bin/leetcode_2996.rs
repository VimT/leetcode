//! 大于等于顺序前缀和的最小缺失整数

pub fn missing_integer(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut i = 1;
    let mut pre_sum = nums[0];
    while i < len && nums[i] == nums[i - 1] + 1 {
        pre_sum += nums[i];
        i += 1;
    }
    for num in pre_sum.. {
        if !nums.contains(&num) {
            return num;
        }
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 2, 5]), 6);
        assert_eq!(func(vec![3, 4, 5, 1, 12, 14, 13]), 15);
    }
    test(missing_integer);
}
