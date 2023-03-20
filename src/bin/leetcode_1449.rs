//! 数位成本和为目标值的最大数字

pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
    let target = target as usize;
    let mut dp = vec![i32::MIN; target + 1];
    let mut char = vec![0; target + 1];
    dp[0] = 0;
    for j in 0..=target {
        for (i, &cost) in cost.iter().enumerate().rev() {
            if j >= cost as usize && dp[j - cost as usize] + 1 > dp[j] {
                dp[j] = dp[j - cost as usize] + 1;
                char[j] = i;
            }
        }
    }
    if dp[target] < 0 {
        return String::from("0");
    }
    let mut result = vec![];
    let mut cur = target;
    while cur > 0 {
        result.push(char[cur] as u8 + b'1');
        cur -= cost[char[cur]] as usize;
    }

    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(cost: Vec<i32>, target: i32) -> String) {
        assert_eq!(func(vec![6, 10, 15, 40, 40, 40, 40, 40, 40], 47), String::from("32211"));
        assert_eq!(func(vec![4, 3, 2, 5, 6, 7, 2, 5, 5], 9), String::from("7772"));
        assert_eq!(func(vec![7, 6, 5, 5, 5, 6, 8, 7, 8], 12), String::from("85"));
        assert_eq!(func(vec![2, 4, 6, 2, 4, 6, 4, 4, 4], 5), String::from("0"));
    }
    test(largest_number);
}
