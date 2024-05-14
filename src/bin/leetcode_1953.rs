//! 你可以工作的最大周数

/// 一个数组，每次将2个下标 -1，问能够减少的最大次数
/// 当 max > sum - max  的时候，必然 max 值不能减完
pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
    let mx = milestones.iter().max().copied().unwrap() as i64;
    let sum = milestones.iter().map(|&x| x as i64).sum::<i64>();
    if sum - mx < mx { (sum - mx) * 2 + 1 } else { sum }
}

fn main() {
    fn test(func: fn(milestones: Vec<i32>) -> i64) {
        assert_eq!(func(vec![5, 7, 5, 7, 9, 7]), 40);
        assert_eq!(func(vec![1, 2, 3]), 6);
        assert_eq!(func(vec![5, 2, 1]), 7);
    }
    test(number_of_weeks);
}
