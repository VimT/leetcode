//! 合并后数组中的最大元素

pub fn max_array_value(nums: Vec<i32>) -> i64 {
    let mut result = 0;
    let mut cur = 0;
    for num in nums.into_iter().rev() {
        if num as i64 <= cur {
            cur += num as i64;
        } else {
            result = result.max(cur);
            cur = num as i64;
        }
    }
    result.max(cur)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![2, 3, 7, 9, 3]), 21);
        assert_eq!(func(vec![5, 3, 3]), 11);
    }
    test(max_array_value);
}
