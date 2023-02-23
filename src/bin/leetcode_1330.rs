//! 翻转子数组得到最大的数组值

/// nextVal=curVal+∣D−B∣+∣C−A∣−∣D−C∣−∣B−A∣，枚举ABCD大小关系的多种情况
pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 1 { return 0; }
    let mut cur = 0;
    for i in 0..len - 1 {
        cur += (nums[i] - nums[i + 1]).abs();
    }
    let mut result = cur;
    // 边界1
    for j in 1..len - 1 {
        result = result.max(cur + (nums[j + 1] - nums[0]).abs() - (nums[j + 1] - nums[j]).abs());
    }
    // 边界2
    for i in 1..len - 1 {
        result = result.max(cur + (nums[i - 1] - nums[len - 1]).abs() - (nums[i] - nums[i - 1]).abs());
    }
    let mut a = nums[0].min(nums[1]);
    let mut b = nums[0].max(nums[1]);
    for i in 0..len - 1 {
        a = a.max(nums[i].min(nums[i + 1]));
        b = b.min(nums[i].max(nums[i + 1]));
    }
    result.max(cur + 2 * (a - b))
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 3, 1, 5, 4]), 10);
        assert_eq!(func(vec![2, 4, 9, 24, 2, 1, 10]), 68);
    }
    test(max_value_after_reverse);
}
