//! 查询排序后的最大公约数

use leetcode::algorithm::cal_prime;
use std::cmp::Ordering;
use std::sync::OnceLock;

pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
    let m = nums.iter().max().copied().unwrap() as usize;
    static PRIME: OnceLock<Vec<usize>> = OnceLock::new();
    let prime = PRIME.get_or_init(|| cal_prime((5. * 1e4) as usize + 1));
    let p = &prime[..prime.binary_search_by(|x| x.cmp(&(m + 1)).then(Ordering::Less)).unwrap_err()];
    // count[i] 表示 gcd 为 i 的数对数量
    let mut count = vec![0; m + 1];
    for a in nums {
        count[a as usize] += 1;
    }
    for &pi in p {
        for j in (0..=m / pi).rev() {
            count[j] += count[j * pi];
        }
    }
    // gcd 为 i 的数中，随机选两个
    for i in 0..=m {
        count[i] = count[i] * (count[i] - 1) / 2;
    }
    // 去重
    for &pi in p {
        for j in 0..=m / pi {
            count[j] -= count[j * pi];
        }
    }
    for i in 1..=m {
        count[i] += count[i - 1];
    }
    queries.into_iter().map(|q| {
        count.binary_search_by(|x| x.cmp(&q).then(Ordering::Less)).unwrap_err() as i32
    }).collect()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32>) {
        assert_eq!(func(vec![2, 3, 4], vec![0, 2, 2]), vec![1, 2, 2]);
        assert_eq!(func(vec![4, 4, 2, 1], vec![5, 3, 1, 0]), vec![4, 2, 1, 1]);
        assert_eq!(func(vec![2, 2], vec![0, 0]), vec![2, 2]);
    }
    test(gcd_values);
}
