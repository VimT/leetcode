//! 重排数组以得到最大前缀分数

pub fn max_score(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut cur = 0;
    let mut result = 0;
    for num in nums.into_iter().rev() {
        cur += num as i64;
        if cur > 0 {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, -1, 0, 1, -3, 3, -3]), 6);
        assert_eq!(func(vec![-2, -3, 0]), 0);
    }
    test(max_score);
}
