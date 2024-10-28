//! 数组的最大因子得分

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * (b / gcd(a, b))
}

pub fn max_score(nums: Vec<i32>) -> i64 {
    if nums.is_empty() { return 0; }
    if nums.len() == 1 { return (nums[0] as i64) * (nums[0] as i64); }

    fn cal(nums: &[i32]) -> i64 {
        if nums.is_empty() { return 0; }
        let a = nums.iter().fold(nums[0] as i64, |acc, &x| gcd(acc, x as i64));
        let b = nums.iter().fold(nums[0] as i64, |acc, &x| lcm(acc, x as i64));
        a * b
    }
    let mut max_score = cal(&nums);
    for i in 0..nums.len() {
        let mut a = nums.clone();
        a.remove(i);
        let score = cal(&a);
        max_score = max_score.max(score);
    }
    max_score
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![2, 4, 8, 16]), 64);
        assert_eq!(func(vec![1, 2, 3, 4, 5]), 60);
        assert_eq!(func(vec![3]), 9);
    }
    test(max_score);
}
