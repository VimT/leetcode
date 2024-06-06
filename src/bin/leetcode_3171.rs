//! 找到按位与最接近 K 的子数组

/// 3097模板
pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut ands = vec![];
    let mut result = u32::MAX;
    for i in 0..len {
        ands.push(i32::MAX);
        let mut j = 0;
        for p in 0..ands.len() {
            ands[p] &= nums[i];
            result = result.min(ands[p].abs_diff(k));
            if ands[j] != ands[p] {
                j += 1;
                ands[j] = ands[p];
            }
        }
        ands.truncate(j + 1);
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 4, 5], 3), 1);
        assert_eq!(func(vec![1, 2, 1, 2], 2), 0);
        assert_eq!(func(vec![1], 10), 9);
    }
    test(minimum_difference);
}
