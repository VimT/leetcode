//! 子数组范围和


pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
    let mut result = 0;
    let len = nums.len();
    for i in 0..len {
        let mut cur_min = nums[i];
        let mut cur_max = nums[i];
        for j in i + 1..len {
            cur_min = cur_min.min(nums[j]);
            cur_max = cur_max.max(nums[j]);
            result += (cur_max - cur_min) as i64;
        }
    }
    result
}

/// 所有子数组的最大值之和sumMax 减去所有子数组的最小值之和sumMin。
/// nums[i] 左侧最近的比它小的数为 nums[j]，右侧最近的比它小的数为nums[k]，那么所有以 nums[i] 为最小值的子数组数目为(k−i)×(i−j)
pub fn sub_array_ranges_single_stack(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut min_left = vec![0; len];
    let mut min_right = vec![0; len];
    let mut max_left = vec![0; len];
    let mut max_right = vec![0; len];
    let mut min_stack = vec![];
    let mut max_stack = vec![];
    for i in 0..len {
        while !min_stack.is_empty() && nums[*min_stack.last().unwrap()] > nums[i] {
            min_stack.pop();
        }
        min_left[i] = min_stack.last().map(|x| *x as i64).unwrap_or(-1);
        min_stack.push(i);
        while !max_stack.is_empty() && nums[*max_stack.last().unwrap()] <= nums[i] {
            max_stack.pop();
        }
        max_left[i] = max_stack.last().map(|x| *x as i64).unwrap_or(-1);
        max_stack.push(i);
    }
    min_stack.clear();
    max_stack.clear();
    for i in (0..len).rev() {
        while !min_stack.is_empty() && nums[*min_stack.last().unwrap()] >= nums[i] {
            min_stack.pop();
        }
        min_right[i] = min_stack.last().map(|x| *x as i64).unwrap_or(len as i64);
        min_stack.push(i);
        while !max_stack.is_empty() && nums[*max_stack.last().unwrap()] < nums[i] {
            max_stack.pop();
        }
        max_right[i] = max_stack.last().map(|x| *x as i64).unwrap_or(len as i64);
        max_stack.push(i);
    }
    let mut sum_max = 0;
    let mut sum_min = 0;
    for i in 0..len {
        sum_max += (max_right[i] - i as i64) * (i as i64 - max_left[i]) * nums[i] as i64;
        sum_min += (min_right[i] - i as i64) * (i as i64 - min_left[i]) * nums[i] as i64;
    }
    sum_max - sum_min
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![1, 2, 3]), 4);
        assert_eq!(func(vec![1, 3, 3]), 4);
        assert_eq!(func(vec![4, -2, -3, 4, 1]), 59);
    }
    test(sub_array_ranges);
    test(sub_array_ranges_single_stack);
}
