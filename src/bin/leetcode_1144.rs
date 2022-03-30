//! 递减元素使数组呈锯齿状

pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut odd = 0;
    let mut even = 0;
    for i in 0..len {
        let min = if i > 0 { nums[i - 1] } else { i32::MAX }.min(if i + 1 < len { nums[i + 1] } else { i32::MAX });
        if nums[i] >= min {
            *if i & 1 == 1 { &mut odd } else { &mut even } += nums[i] + 1 - min;
        }
    }
    odd.min(even)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3]), 2);
        assert_eq!(func(vec![9, 6, 1, 6, 2]), 4);
    }
    test(moves_to_make_zigzag);
}
