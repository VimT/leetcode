//! 元素值大于变化阈值的子数组

/// 看提示的，用单调栈找 一个数作为子数组的最小数，子数组的最大长度能到多少
pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
    let len = nums.len();
    let mut left_min = vec![0; len];
    let mut right_min = vec![len; len];
    let mut s = vec![];
    for i in 0..len {
        while !s.is_empty() && nums[*s.last().unwrap()] > nums[i] {
            right_min[s.pop().unwrap()] = i;
        }
        s.push(i);
    }
    for i in (0..len).rev() {
        while !s.is_empty() && nums[*s.last().unwrap()] > nums[i] {
            left_min[s.pop().unwrap()] = i + 1;
        }
        s.push(i);
    }
    for i in 0..len {
        let len = right_min[i] - left_min[i];
        if nums[i] > threshold / len as i32 {
            return len as i32;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, threshold: i32) -> i32) {
        assert_eq!(func(vec![1, 3, 4, 3, 1], 6), 3);
        assert_eq!(func(vec![6, 5, 6, 5, 8], 7), 5);
    }
    test(valid_subarray_size);
}
