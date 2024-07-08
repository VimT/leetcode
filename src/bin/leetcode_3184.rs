//! 构成整天的下标对数目 I

pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
    let len = hours.len();
    let mut result = 0;
    for i in 0..len {
        for j in i + 1..len {
            if (hours[i] + hours[j]) % 24 == 0 {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(hours: Vec<i32>) -> i32) {
        assert_eq!(func(vec![12, 12, 30, 24, 24]), 2);
        assert_eq!(func(vec![72, 48, 24, 3]), 3);
    }
    test(count_complete_day_pairs);
}
