//! 重排水果

use std::collections::HashMap;
use leetcode::algorithm::nth_element;

pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
    let mut cnt: HashMap<i64, (i32, i32)> = HashMap::new();
    for num in basket1 {
        cnt.entry(num as i64).or_default().0 += 1;
    }
    for num in basket2 {
        cnt.entry(num as i64).or_default().1 += 1;
    }
    let mut b1 = vec![];
    let mut b2 = vec![];
    let mut min = i64::MAX;
    for (num, (left, right)) in cnt {
        min = min.min(num);
        if (left + right) & 1 == 1 {
            return -1;
        }
        let mid = (left + right) / 2;
        if left > mid {
            for _ in 0..left - mid {
                b1.push(num);
            }
        } else if right > mid {
            for _ in 0..right - mid {
                b2.push(num);
            }
        }
    }
    b1.sort_unstable();
    b2.sort_unstable();
    let len = b1.len();
    let mut result = 0;
    for i in 0..len {
        result += b1[i].min(b2[len - 1 - i]).min(min * 2);
    }
    result
}

/// 空间优化：
/// 1. 不需要 2 个counter
/// 2. 只需要 1 个数组：因为最后是 b1取前半个，b2取后半个
/// 3. 不用数组排序，用快速选择 （选最小的k个数），可以O(n)
pub fn min_cost2(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
    let mut cnt: HashMap<i64, i32> = HashMap::new();
    for num in basket1 {
        *cnt.entry(num as i64).or_default() += 1;
    }
    for num in basket2 {
        *cnt.entry(num as i64).or_default() -= 1;
    }
    let mut a = vec![];
    let mut mn = i64::MAX;
    for (num, c) in cnt {
        mn = mn.min(num);
        if c & 1 == 1 { return -1; }
        for _ in 0..c.abs() / 2 {
            a.push(num);
        }
    }
    let len = a.len();
    nth_element(&mut a, len / 2);
    a[..len / 2].iter().map(|&x| x.min(mn + mn)).sum()
}

fn main() {
    fn test(func: fn(basket1: Vec<i32>, basket2: Vec<i32>) -> i64) {
        assert_eq!(func(vec![84, 80, 43, 8, 80, 88, 43, 14, 100, 88], vec![32, 32, 42, 68, 68, 100, 42, 84, 14, 8]), 48);
        assert_eq!(func(vec![4, 2, 2, 2], vec![1, 4, 1, 2]), 1);
        assert_eq!(func(vec![2, 3, 4, 1], vec![3, 2, 5, 1]), -1);
        assert_eq!(func(vec![5, 8, 15, 7], vec![5, 7, 8, 15]), 0);
    }
    test(min_cost);
    test(min_cost2);
}
