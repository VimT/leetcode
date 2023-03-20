//! 和有限的最长子序列

pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    queries.into_iter().map(|x| {
        nums.binary_search(&x).map(|x| x + 1).unwrap_or_else(|x| x) as i32
    }).collect()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![4, 5, 2, 1], vec![3, 10, 21]), vec![2, 3, 4]);
        assert_eq!(func(vec![2, 3, 4, 5], vec![1]), vec![0]);
    }
    test(answer_queries);
}
