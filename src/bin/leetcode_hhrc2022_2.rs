//! 题目-02. 销售出色区间


use std::collections::BTreeMap;

/// 找 最大>8的区间长度，(-1,1)数组求和为正的最长子数组长度
/// 表面“最大区间”问题可以用二分，其实不能用，因为解不是连续的，小区间不符合要求并不意味着大区间不符合要求
/// 用BTreeMap 存储前缀和，BTreeMap遍历过程中，result = max(result, 当前presum最大index - 前面prevsum的最小index)
pub fn longest_esr(sales: Vec<i32>) -> i32 {
    let mut prev: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    let mut cursum = 0;
    prev.insert(0, vec![0]);
    for (i, &v) in sales.iter().enumerate() {
        cursum += if v > 8 { 1 } else { -1 };
        prev.entry(cursum).or_default().push(i + 1);
    }
    let mut result = 0;
    let mut mn = 1e9 as usize;
    for (_, v) in prev {
        if *v.last().unwrap() > mn {
            result = result.max(*v.last().unwrap() - mn);
        }
        mn = mn.min(v[0]);
    }
    result as i32
}

/// 设 f(i) 为 数组[0, i)前缀和.题目就是找 i < j, f(j + 1) - f(i) > 0
/// 所以固定 i，找满足要求的最大j。或者是固定j，找满足要求的最小i。
/// 可以单调队列+二分查找。 或者双指针
pub fn longest_esr2(sales: Vec<i32>) -> i32 {
    let len = sales.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + if sales[i] > 8 { 1 } else { -1 };
    }
    let mut mx = vec![0; len + 1];
    mx[len] = presum[len];
    for i in (0..len).rev() {
        mx[i] = mx[i + 1].max(presum[i]);
    }
    let mut result = 0;
    let mut j = 1;
    for i in 0..=len {
        // 后面最大的presum都比当前的小，跳过
        if presum[i] >= mx[j] {
            continue;
        }
        // j向后遍历，直到 j后面的presum都比presum[i]小
        while j <= len && presum[i] < mx[j] {
            j += 1;
        }
        if j > i {
            result = result.max(j - i - 1);
        }
        if j > len { break; }
    }
    result as i32
}

/// 单调队列+二分查找
pub fn longest_esr3(sales: Vec<i32>) -> i32 {
    let len = sales.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + if sales[i] > 8 { 1 } else { -1 };
    }
    let mut arr = vec![]; // 单调减队列
    let mut result = 0;
    for i in 0..len {
        if arr.is_empty() || presum[*arr.last().unwrap()] > presum[i] {
            arr.push(i);
        }
        // 二分找 < presum[j+1]的最小i
        let mut left = 0;
        let mut right = arr.len();
        while left < right {
            let mid = (left + right) / 2;
            if presum[arr[mid]] >= presum[i + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if left < arr.len() {
            result = result.max(i + 1 - arr[left]);
        }
    }
    result as i32
}

fn main() {
    fn test(func: fn(sales: Vec<i32>) -> i32) {
        assert_eq!(func(vec![9, 9, 9]), 3);
        assert_eq!(func(vec![10, 2, 1, 4, 3, 9, 6, 9, 9]), 5);
        assert_eq!(func(vec![5, 6, 7]), 0);
    }
    test(longest_esr);
    test(longest_esr2);
    test(longest_esr3);
}
