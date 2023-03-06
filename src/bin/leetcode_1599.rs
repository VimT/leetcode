//! 经营摩天轮的最大利润

pub fn min_operations_max_profit(customers: Vec<i32>, boarding_cost: i32, running_cost: i32) -> i32 {
    let mut wait = 0;
    let mut max = 0;
    let mut result = -1;
    let mut cur = 0;
    let mut i = 0;
    for custom in customers {
        wait += custom;
        let up = wait.min(4);
        wait -= up;
        cur += up * boarding_cost - running_cost;
        i += 1;
        if cur > max {
            max = cur;
            result = i as i32;
        }
    }
    let t = wait / 4;
    i += t;
    wait %= 4;
    cur += t * (4 * boarding_cost - running_cost);
    if cur > max {
        max = cur;
        result = i as i32;
    }
    i += 1;
    cur += wait * boarding_cost - running_cost;
    if cur > max {
        result = i as i32;
    }
    result
}

fn main() {
    fn test(func: fn(customers: Vec<i32>, boarding_cost: i32, running_cost: i32) -> i32) {
        assert_eq!(func(vec![10, 10, 6, 4, 7], 3, 8), 9);
        assert_eq!(func(vec![8, 3], 5, 6), 3);
        assert_eq!(func(vec![10, 9, 6], 6, 4), 7);
        assert_eq!(func(vec![3, 4, 0, 5, 1], 1, 92), -1);
    }
    test(min_operations_max_profit);
}
