//! 构造字符串的总得分和

use leetcode::suffix_array::SuffixArray;

/// 扩展kmp，z函数
pub fn sum_scores_z(s: String) -> i64 {
    let s = s.as_bytes();
    let len = s.len();
    let mut z = vec![0; len];
    let mut l = 0;
    let mut r = 0;
    let mut result = len;
    for i in 1..len {
        if i <= r && z[i - l] < r - i + 1 {
            z[i] = z[i - l];
        } else {
            z[i] = if r > i - 1 { r + 1 - i } else { 0 };
            while i + z[i] < len && s[z[i]] == s[i + z[i]] {
                z[i] += 1;
            }
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
        result += z[i];
    }
    result as i64
}

/// 后缀数组
/// lcp[sa[i], sa[j]] = min(height[i+1..j])
pub fn sum_scores_sa(s: String) -> i64 {
    let s = s.as_bytes();
    let len = s.len();
    let sa = SuffixArray::new(s);
    let height = sa.get_height();
    let start = sa.ranks[0];
    let mut min = 1e9 as usize;
    let mut result = len;
    for i in start + 1..len {
        min = min.min(height[i]);
        result += min;
    }
    min = height[start];
    for i in (0..start).rev() {
        result += min;
        min = min.min(height[i]);
    }
    result as i64
}

/// 字符串hash（滚动hash） + 二分查找
pub fn sum_scores_bs(s: String) -> i64 {
    const BASE: u64 = 1331;
    const MOD: u64 = 1e9 as u64 + 7;
    let s = s.as_bytes();
    let len = s.len();
    let mut pre = vec![0; len + 1];
    let mut pw = vec![1; len + 1];
    for i in 1..=len {
        pre[i] = (pre[i - 1] * BASE + (s[i - 1] - b'a') as u64) % MOD;
        pw[i] = pw[i - 1] * BASE % MOD;
    }
    let mut result = 0;
    for i in 0..len {
        let mut left = 1;
        let mut right = len - i + 1;
        while left < right {
            let mid = (left + right) >> 1;
            // 如果 0-mid 的hash == i-i+mid 的hash，说明可以更大
            if pre[mid] == (pre[i + mid] + MOD - pre[i] * pw[mid] % MOD) % MOD {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        result += left - 1;
    }
    result as i64
}


fn main() {
    fn test(func: fn(s: String) -> i64) {
        assert_eq!(func(String::from("babab")), 9);
        assert_eq!(func(String::from("azbazbzaz")), 14);
    }
    test(sum_scores_z);
    test(sum_scores_bs);
    test(sum_scores_sa);
}
