//! 每个小孩最多能分到多少糖果

pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let mut left = 1;
    let mut right = *candies.iter().max().unwrap() + 1;
    while left < right {
        let mid = (left + right) / 2;
        let mut people = 0;
        for &cand in &candies {
            people += (cand / mid) as i64;
        }
        if people >= k {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left - 1
}

fn main() {
    assert_eq!(maximum_candies(vec![5, 8, 6], 3), 5);
    assert_eq!(maximum_candies(vec![2, 5], 11), 0);
    assert_eq!(maximum_candies(vec![1, 2, 3, 4, 10], 5), 3);
}
