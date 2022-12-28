//! 礼盒的最大甜蜜度

pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
    price.sort_unstable();
    let len = price.len();
    let mut left = 0;
    let mut right = price[len - 1] - price[0] + 1;
    while left < right {
        let mid = (left + right) >> 1;
        let mut last = price[0];
        let mut cnt = 1;
        for i in 1..len {
            if price[i] - last >= mid {
                cnt += 1;
                last = price[i];
                if cnt >= k { break; }
            }
        }
        if cnt >= k {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left - 1
}

fn main() {
    fn test(func: fn(price: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 3, 1], 2), 2);
        assert_eq!(func(vec![13, 5, 1, 8, 21, 2], 3), 8);
        assert_eq!(func(vec![7, 7, 7, 7], 2), 0);
    }
    test(maximum_tastiness);
}
