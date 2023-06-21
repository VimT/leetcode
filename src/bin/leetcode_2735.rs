//! 收集巧克力

pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
    let len = nums.len();
    let mut cost = vec![0; len];
    for i in 0..len {
        let mut cur_min = nums[i] as i64;
        for j in 0..len {
            cur_min = cur_min.min(nums[(len + i - j) % len] as i64);
            cost[j] += cur_min;
        }
    }
    let mut result = i64::MAX;
    for i in 0..len {
        result = result.min(x as i64 * i as i64 + cost[i]);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, x: i32) -> i64) {
        assert_eq!(func(vec![20, 1, 15], 5), 13);
        assert_eq!(func(vec![1, 2, 3], 4), 6);
    }
    test(min_cost);
}
