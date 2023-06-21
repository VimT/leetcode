//! 移动机器人

/// 对撞后往相反的方向走，相当于没有撞
pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
    let len = nums.len();
    let s = s.as_bytes();
    let mut nums: Vec<i64> = nums.into_iter().zip(0..).map(|(num, i)| {
        if s[i] == b'R' { num as i64 + d as i64 } else { num as i64 - d as i64 } }).collect();

    nums.sort_unstable();
    let mut presum = 0;
    let mut result = 0;
    const MOD: i64 = 1e9 as i64 + 7;
    for i in 1..len {
        let diff = nums[i] - nums[i - 1];
        presum += diff * i as i64;
        result = (result + presum) % MOD;
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, s: String, d: i32) -> i32) {
        assert_eq!(func(vec![-2, 0, 2], String::from("RLL"), 3), 8);
        assert_eq!(func(vec![1, 0], String::from("RL"), 2), 5);
    }
    test(sum_distance);
}
