//! 分割数组的方案数

pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let sum: i64 = nums.iter().map(|x| *x as i64).sum();
    let mut cur_sum = 0;
    for &num in &nums[..nums.len() - 1] {
        cur_sum += num as i64;
        if cur_sum >= sum - cur_sum {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![10, 4, -8, 7]), 2);
        assert_eq!(func(vec![2, 3, 1, 0]), 2);
    }
    test(ways_to_split_array);
}
