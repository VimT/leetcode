//! 好子数组的最大分数

/// 中间扩散
pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut i = (k - 1) as usize;
    let mut j = (k + 1) as usize;
    let mut result = 0;
    let mut min = nums[k as usize];
    loop {
        while i < len && nums[i] >= min { i = i.wrapping_sub(1); }
        while j < len && nums[j] >= min { j += 1; }
        result = result.max(min * (j - i.wrapping_add(1)) as i32);
        if i >= len && j >= len { break; }
        min = (if i < len { nums[i] } else { 0 }).max(if j < len { nums[j] } else { 0 });
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 4, 3, 7, 4, 5], 3), 15);
        assert_eq!(func(vec![5, 5, 4, 5, 4, 1, 1, 1], 0), 20);
    }
    test(maximum_score);
}
