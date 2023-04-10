//! 使数组元素全部相等的最少操作次数

pub fn min_operations(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
    nums.sort_unstable();
    let len = nums.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + nums[i] as i64;
    }
    queries.into_iter().map(|query| {
        let split = nums.binary_search(&query).unwrap_or_else(|x| x);
        query as i64 * split as i64 - presum[split] + presum[len] - presum[split] - query as i64 * (len - split) as i64
    }).collect()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64>) {
        assert_eq!(func(vec![3, 1, 6, 8], vec![1, 5]), vec![14, 10]);
        assert_eq!(func(vec![2, 9, 6, 3], vec![10]), vec![20]);
    }
    test(min_operations);
}
