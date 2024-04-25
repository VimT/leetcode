//! 单面值组合的第 K 小金额

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// 二分查找 + 容斥原理
pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
    let mut left = 1;
    let mx_coin = coins.iter().max().copied().unwrap() as i64;
    let k = k as i64;
    let mut right = k * mx_coin;
    let mut cal = vec![];
    for i in 1usize..1 << coins.len() {
        let mut lcm = 1;
        for (j, &x) in coins.iter().enumerate() {
            if i >> j & 1 == 1 {
                lcm = lcm / gcd(lcm, x as i64) * x as i64;
            }
        }
        cal.push(lcm * (if i.count_ones() % 2 == 1 { 1 } else { -1 }));
    }

    while left < right {
        let mid = (left + right) / 2;
        let cnt = cal.iter().map(|&x| mid / x).sum::<i64>();
        if cnt < k {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn main() {
    fn test(func: fn(coins: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![6, 1, 2, 4], 4), 4);
        assert_eq!(func(vec![5, 2], 7), 12);
        assert_eq!(func(vec![3, 6, 9], 3), 9);
    }
    test(find_kth_smallest);
}
