//! 序列中不同最大公约数的数目

use leetcode::gcd::gcd3;

/// 从值域的角度看：一个数是不是一串子序列的gcd
pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
    let max = *nums.iter().max().unwrap() as usize;
    let mut has = vec![false; max + 1];
    let mut result = 0;
    for num in nums {
        has[num as usize] = true;
    }
    for i in 1..=max {
        let mut g = 0;
        for j in (i..=max).step_by(i) {
            if has[j] {
                g = gcd3(g, j as i32);
                if g == i as i32 {
                    result += 1;
                    break;
                }
            }
        }
    }
    result
}

/// 优化：1..=max/3
pub fn count_different_subsequence_gc_ds2(nums: Vec<i32>) -> i32 {
    let max = *nums.iter().max().unwrap() as usize;
    let mut has = vec![false; max + 1];
    let mut result = 0;
    for num in nums {
        if !has[num as usize] {
            has[num as usize] = true;
            result += 1;
        }
    }
    for i in 1..=max / 3 {
        if !has[i] {
            let mut g = 0;
            let mut j = i << 1;
            // 不要用 (1..=max).step_by()  step_by会挨个遍历过去
            while j <= max {
                if has[j] {
                    g = gcd3(g, j as i32);
                    if g == i as i32 {
                        result += 1;
                        break;
                    }
                }
                j += i;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![6, 10, 3]), 5);
        assert_eq!(func(vec![5, 15, 40, 5, 6]), 7);
    }
    test(count_different_subsequence_gc_ds);
    test(count_different_subsequence_gc_ds2);
}
