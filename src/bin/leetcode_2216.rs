//! 美化数组的最少删除数

/// 贪心
pub fn min_deletion(nums: Vec<i32>) -> i32 {
    let mut s = Vec::with_capacity(nums.len());
    let mut result = 0;
    for num in nums {
        if s.len() & 1 == 1 && num == *s.last().unwrap() {
            result += 1;
            continue;
        }
        s.push(num);
    }
    result + (s.len() & 1) as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 1, 2, 3, 5]), 1);
        assert_eq!(func(vec![1, 1, 2, 2, 3, 3]), 2);
    }
    test(min_deletion);
}
