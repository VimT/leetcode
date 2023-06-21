//! 存在连续三个奇数的数组

pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    let mut odd_cnt = 0;
    for num in arr {
        if num & 1 == 1 {
            odd_cnt += 1;
            if odd_cnt == 3 { return true; }
        } else {
            odd_cnt = 0;
        }
    }
    false
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> bool) {
        assert_eq!(func(vec![2, 6, 4, 1]), false);
        assert_eq!(func(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]), true);
    }
    test(three_consecutive_odds);
}
