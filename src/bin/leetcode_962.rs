//!  最大宽度坡

/// 单调栈：存的是从 A[0] 开始的递减序列
pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut s = Vec::with_capacity(len);
    for i in 0..len {
        if s.is_empty() || nums[*s.last().unwrap()] > nums[i] {
            s.push(i);
        }
    }
    let mut result = 0;
    for i in (0..len).rev() {
        while !s.is_empty() && nums[*s.last().unwrap()] <= nums[i] {
            result = result.max(i - s.pop().unwrap())
        }
    }
    result as i32
}

fn main() {
    assert_eq!(max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
    assert_eq!(max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]), 7);
}
