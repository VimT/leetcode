//! 找出分数最低的排列

/// 比赛做法：简单剪了个枝就过了
/// 优化：分数是一个循环的形式，因此对于任意一个排列，把它左移任意次后得到的排列分数仍然相同。因为题目要求字典序最小的排列，所以排列的第一位肯定是 0
pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut perm = vec![0; len];
    let mut ans = vec![0; len];
    let mut ans_score = i32::MAX;

    fn back(nums: &Vec<i32>, perm: &mut Vec<i32>, i: usize, visited: i32, score: i32, ans: &mut Vec<i32>, ans_score: &mut i32) {
        if i == nums.len() {
            let score = score + perm[nums.len() - 1].abs_diff(nums[perm[0] as usize]) as i32;
            if score < *ans_score {
                *ans_score = score;
                ans.copy_from_slice(perm);
            }
            return;
        }
        // 对第i位，尝试填j
        for j in 0..nums.len() {
            if visited >> j & 1 == 0 {
                perm[i] = j as i32;
                let new_score = score + perm[i - 1].abs_diff(nums[j]) as i32;
                if new_score > *ans_score { continue; }
                back(nums, perm, i + 1, visited | (1 << j), new_score, ans, ans_score);
            }
        }
    }
    // 肯定是0开头
    back(&nums, &mut perm, 1, 1, 0, &mut ans, &mut ans_score);
    ans
}

/// 哈密顿回路可以从 0 处拆开，变成哈密顿路径，
pub fn find_permutation2(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let m = len - 1;
    let mm = 1 << m;
    let mut dp = vec![vec![(i32::MAX / 2, len); m]; mm]; // dp[mask][i] 表示 mask 状态下，以 i 结尾的最小分数和前一个数的索引
    for i in 1..mm {
        let mut j = i;
        while j > 0 {
            let u = j.trailing_zeros() as usize;
            let s = i - (1 << u);
            if s == 0 {
                dp[i][u] = ((u as i32 + 1 - nums[0]).abs(), len);
            } else {
                let mut min_sum = i32::MAX;
                let mut min_ver = len;
                let mut k = s;
                while k > 0 {
                    let v = k.trailing_zeros() as usize;
                    let sum = dp[s][v].0 + (u as i32 + 1 - nums[v + 1]).abs() + if i == mm - 1 { nums[u + 1] } else { 0 };
                    if sum < min_sum {
                        min_sum = sum;
                        min_ver = v;
                    }
                    k -= 1 << v;
                }
                dp[i][u] = (min_sum, min_ver);
            }
            j -= 1 << u;
        }
    }

    let mut min_sum = i32::MAX;
    let mut min_ver = len;
    for i in 0..m {
        if dp[mm - 1][i].0 < min_sum {
            min_sum = dp[mm - 1][i].0;
            min_ver = i;
        }
    }
    let mut result = vec![0];
    let mut s = mm - 1;
    let mut i = min_ver;
    while i != len {
        result.push(i as i32 + 1);
        let j = dp[s][i].1;
        s -= 1 << i;
        i = j;
    }
    result
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![0, 1, 3, 2]), vec![0, 1, 3, 2]);
        assert_eq!(func(vec![2, 1, 0]), vec![0, 1, 2]);
        assert_eq!(func(vec![0, 2, 1]), vec![0, 2, 1]);
        assert_eq!(func(vec![1, 0, 2]), vec![0, 1, 2]);
    }
    test(find_permutation);
    test(find_permutation2);
}
