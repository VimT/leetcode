//! 最小元素和最大元素的最小平均值


pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
    nums.sort_unstable();
    let mut result: f64 = 100.0;
    let len = nums.len();
    for i in 0..len / 2 {
        result = result.min((nums[i] + nums[len - 1 - i]) as f64 / 2.0);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> f64) {
        assert_eq!(func(vec![7, 8, 3, 4, 15, 13, 4, 1]), 5.5);
        assert_eq!(func(vec![1, 9, 8, 3, 10, 5]), 5.5);
        assert_eq!(func(vec![1, 2, 3, 7, 8, 9]), 5.0);
    }
    test(minimum_average);
}
