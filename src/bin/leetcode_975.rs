//! 奇偶跳

use std::collections::BTreeMap;

pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut odd_jump = vec![0; len];
    let mut even_jump = vec![0; len];
    let mut arr_inc: Vec<usize> = (0..len).collect();
    let mut arr_dec: Vec<usize> = (0..len).collect();
    // 先排序，对排序后的索引使用 单调栈，找 >= 当前数的 最小的索引
    // 注意这里要用稳定排序
    arr_inc.sort_by_key(|x| arr[*x]);
    arr_dec.sort_by_key(|x| -arr[*x]);
    let mut s = Vec::with_capacity(len);
    for i in arr_inc {
        while !s.is_empty() && *s.last().unwrap() < i {
            odd_jump[s.pop().unwrap()] = i;
        }
        s.push(i);
    }
    s.clear();
    for i in arr_dec {
        while !s.is_empty() && *s.last().unwrap() < i {
            even_jump[s.pop().unwrap()] = i;
        }
        s.push(i);
    }
    let mut dp = vec![[false, false]; len];
    dp[len - 1] = [true, true];
    for i in (0..len).rev() {
        if odd_jump[i] > 0 {
            dp[i][0] = dp[odd_jump[i]][1];
        }
        if even_jump[i] > 0 {
            dp[i][1] = dp[even_jump[i]][0];
        }
    }
    let mut result = 0;
    for i in 0..len {
        if dp[i][0] {
            result += 1;
        }
    }
    result
}

/// 用有序dict快速找大于和小于的数
pub fn odd_even_jumps_optimise(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    if len <= 1 { return len as i32; }
    let mut odd = vec![false; len];
    let mut even = vec![false; len];
    odd[len - 1] = true;
    even[len - 1] = true;
    let mut vals = BTreeMap::new();
    vals.insert(arr[len - 1], len - 1);
    for i in (0..len - 1).rev() {
        let v = arr[i];
        if let Some(&vv) = vals.get(&v) {
            odd[i] = even[vv];
            even[i] = odd[vv];
        } else {
            if let Some((_, &lower)) = vals.range(..v).last() {
                even[i] = odd[lower];
            }
            if let Some((_, &higher)) = vals.range(v + 1..).next() {
                odd[i] = even[higher];
            }
        }
        vals.insert(v, i);
    }
    odd.into_iter().filter(|x| *x).count() as i32
}

fn main() {
    assert_eq!(odd_even_jumps_optimise(vec![31, 28, 8, 41, 20, 28, 41, 26, 46, 30, 41, 34, 2, 29, 24, 39, 28, 11, 22, 21, 33, 15, 14, 35, 49, 23, 15, 46, 19, 28, 13, 19, 38]), 21);
    assert_eq!(odd_even_jumps_optimise(vec![1, 2, 3, 2, 1, 4, 4, 5]), 6);
    assert_eq!(odd_even_jumps_optimise(vec![10, 13, 12, 14, 15]), 2);
    assert_eq!(odd_even_jumps_optimise(vec![2, 3, 1, 1, 4]), 3);
    assert_eq!(odd_even_jumps_optimise(vec![5, 1, 3, 4, 2]), 3);
}
