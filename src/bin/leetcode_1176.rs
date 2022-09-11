//! 健身计划评估

pub fn diet_plan_performance(calories: Vec<i32>, k: i32, lower: i32, upper: i32) -> i32 {
    let len = calories.len();
    let mut sum = 0;
    let k = k as usize;
    for i in 0..k {
        sum += calories[i];
    }
    let mut result = 0;
    for i in 0..=len - k {
        if sum < lower {
            result -= 1;
        }
        if sum > upper {
            result += 1;
        }
        if i < len - k {
            sum += calories[i + k];
            sum -= calories[i];
        }
    }
    result
}

fn main() {
    fn test(func: fn(calories: Vec<i32>, k: i32, lower: i32, upper: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4, 5], 1, 3, 3), 0);
        assert_eq!(func(vec![3, 2], 2, 0, 1), 1);
        assert_eq!(func(vec![6, 5, 0, 0], 2, 1, 5), 0);
    }
    test(diet_plan_performance);
}
