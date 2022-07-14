//! 有效子数组的数目

/// 单调栈
pub fn valid_subarrays(nums: Vec<i32>) -> i32 {
    let mut s = vec![];
    let mut result = 0;
    for num in nums {
        while !s.is_empty() && *s.last().unwrap() > num {
            s.pop();
        }
        s.push(num);
        result += s.len() as i32;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 4, 2, 5, 3]), 11);
        assert_eq!(func(vec![3, 2, 1]), 3);
        assert_eq!(func(vec![2, 2, 2]), 6);
    }
    test(valid_subarrays);
}
