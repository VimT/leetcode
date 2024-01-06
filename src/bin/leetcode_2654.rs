//! 使数组所有元素变成 1 的最少操作次数

use leetcode::gcd::gcd;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut g = 0;
    let mut more1 = 0;
    let len = nums.len();
    for &num in &nums {
        g = gcd(g, num);
        if num > 1 { more1 += 1; }
    }
    if g > 1 { return -1; }
    if more1 != len { return more1 as i32; }
    let mut min_size = len;  // 最小的操作出1的连续子数组
    for i in 0..len {
        let mut g = 0;
        for j in i..len {
            g = gcd(g, nums[j]);
            if g == 1 {
                min_size = min_size.min(j - i + 1);
                break;
            }
        }
    }
    (min_size + len - 2) as i32
}

/// gcd值要么相同，要么减半， 所以可以用 O(nlogU) 的时间复杂度算出
pub fn min_operations2(nums: Vec<i32>) -> i32 {
    let mut g = 0;
    let mut more1 = 0;
    let len = nums.len();
    for &num in &nums {
        g = gcd(g, num);
        if num > 1 { more1 += 1; }
    }
    if g > 1 { return -1; }
    if more1 != len { return more1 as i32; }
    let mut min_size = len;
    let mut g = vec![]; // g[i] = (x, i) ，表示 gcd为x的，出现的最晚的位置
    for (i, num) in nums.into_iter().enumerate() {
        g.push((num, i));
        // 去重
        let mut j = 0;
        for i in 0..g.len() {
            g[i].0 = gcd(g[i].0, num);
            if g[j].0 != g[i].0 {
                j += 1;
                g[j] = g[i];
            } else {
                g[j].1 = g[i].1;
            }
        }
        g.truncate(j + 1);
        if g[0].0 == 1 {
            min_size = min_size.min(i - g[0].1 + 1);
        }
    }
    (min_size + len - 2) as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 6, 3, 4]), 4);
        assert_eq!(func(vec![2, 10, 6, 14]), -1);
    }
    test(min_operations);
    test(min_operations2);
}
