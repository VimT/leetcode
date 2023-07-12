//! 乘积为正数的最长子数组长度

/// 朴素思路：按0分割成段，如果段积为负，段的最长正数子数组 = max(右边 - 左边第一个负 - 1, 右边第一个负 - 左边)
pub fn get_max_len(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut i = 0;
    let mut result = 0;
    while i < len {
        if nums[i] == 0 {
            i += 1;
            continue;
        }
        let mut cur = nums[i] > 0;
        let mut j = i + 1;
        let mut first_neg = if !cur { Some(i) } else { None };
        let mut last_neg = i;
        while j < len && nums[j] != 0 {
            cur = !(cur ^ (nums[j] > 0));
            if nums[j] < 0 {
                if first_neg.is_none() { first_neg = Some(j); }
                last_neg = j;
            }
            j += 1;
        }
        if cur {
            result = result.max(j - i)
        } else {
            result = result.max(last_neg - i);
            if let Some(x) = first_neg {
                result = result.max(j - x - 1);
            }
        }

        i = j;
    }
    result as i32
}

/// 动态规划：pos[i], neg[i] 表示以i下标结尾的最长正数子数组和最长负数子数组
pub fn get_max_len2(nums: Vec<i32>) -> i32 {
    let mut pos = (nums[0] > 0) as i32;
    let mut neg = (nums[0] < 0) as i32;
    let mut result = pos;
    for &num in &nums[1..] {
        if num > 0 {
            pos += 1;
            neg = if neg > 0 { neg + 1 } else { 0 };
        } else if num < 0 {
            let new_pos = if neg > 0 { neg + 1 } else { 0 };
            let new_neg = pos + 1;
            pos = new_pos;
            neg = new_neg;
        } else {
            pos = 0;
            neg = 0;
        }
        result = result.max(pos);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![25, 10, -28, -12, -13, -16, -13, 28, 5, 21, 28, 4, 0, -1]), 9);
        assert_eq!(func(vec![1, -2, -3, 4]), 4);
        assert_eq!(func(vec![0, 1, -2, -3, -4]), 3);
        assert_eq!(func(vec![-1, -2, -3, 0, 1]), 2);
    }
    test(get_max_len);
    test(get_max_len2);
}
