//! 匹配模式数组的子数组数目 II

use leetcode::algorithm::max_match_length;

/// nums 的相邻关系转换为 [-1, 0, 1] 的数组，然后求 pattern 的出现次数。 KMP
pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut q = vec![0; len - 1];
    for i in 0..len - 1 {
        q[i] = (nums[i + 1] - nums[i]).signum();
    }
    let n = q.len();
    let m = pattern.len();
    if m == 0 { return 0; }
    let next = max_match_length(&pattern);
    let mut j = 0;
    let mut result = 0;
    for i in 0..n {
        while j > 0 && q[i] != pattern[j] {
            j = next[j - 1];
        }
        if q[i] == pattern[j] { j += 1; }
        if j == m {
            result += 1;
            j = next[j - 1];
        }
    }
    result
}

/// Z函数做法
pub fn count_matching_subarrays2(nums: Vec<i32>, mut pattern: Vec<i32>) -> i32 {
    let m = pattern.len();
    pattern.push(2);
    for win in nums.windows(2) {
        pattern.push((win[1] - win[0]).signum());
    }
    let n = pattern.len();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        if i <= r {
            z[i] = z[i - l].min(r + 1 - i);
        }
        while i + z[i] < n && pattern[z[i]] == pattern[i + z[i]] {
            l = i;
            r = i + z[i];
            z[i] += 1;
        }
    }
    z[m + 1..].iter().filter(|&&x| x == m).count() as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, pattern: Vec<i32>) -> i32) {
        assert_eq!(func(vec![645814052, 330048419, 645814052, 330048419, 160218431, 46882695], vec![-1, 1, -1, -1]), 1);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6], vec![1, 1]), 4);
        assert_eq!(func(vec![1, 4, 4, 1, 3, 5, 5, 3], vec![1, 0, -1]), 2);
    }
    test(count_matching_subarrays);
    test(count_matching_subarrays2);
}
