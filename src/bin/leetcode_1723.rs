//! 完成所有工作的最短时间

/// 朴素的 DFS + 最大化剪枝效果，并且尽量让 k 份平均: 任务优先分配给「空闲工人」
pub fn minimum_time_required_dfs(jobs: Vec<i32>, k: i32) -> i32 {
    fn dfs(jobs: &Vec<i32>, used: usize, idx: usize, k: usize, cur: &mut Vec<i32>, max: i32, result: &mut i32) {
        if max >= *result {
            return;
        }
        if idx == jobs.len() {
            *result = max;
            return;
        }
        if used < k {
            cur[used] = jobs[idx];
            dfs(jobs, used + 1, idx + 1, k, cur, max.max(cur[used]), result);
            cur[used] = 0;
        }
        for i in 0..used {
            cur[i] += jobs[idx];
            dfs(jobs, used, idx + 1, k, cur, max.max(cur[i]), result);
            cur[i] -= jobs[idx];
        }
    }
    let mut result = i32::MAX;
    let mut cur = vec![0; k as usize];
    dfs(&jobs, 0, 0, k as usize, &mut cur, 0, &mut result);
    result
}

pub fn minimum_time_required_binary_search(jobs: Vec<i32>, k: i32) -> i32 {
    fn dfs(jobs: &Vec<i32>, idx: usize, cur: &mut Vec<i32>, k: usize, target: i32) -> bool {
        if idx == jobs.len() {
            return true;
        }
        for i in 0..k {
            if cur[i] + jobs[idx] <= target {
                cur[i] += jobs[idx];
                if dfs(jobs, idx + 1, cur, k, target) { return true; }
                cur[i] -= jobs[idx];
            }
            if cur[i] == 0 || cur[i] + jobs[idx] == target {
                break;
            }
        }
        false
    }
    let mut left = 0;
    let mut right = 0;
    for &i in &jobs {
        left = left.max(i);
        right += i;
    }
    while left < right {
        let mid = (left + right) >> 1;
        dbg!(left, right, mid);
        let mut cur = vec![0; k as usize];
        if dfs(&jobs, 0, &mut cur, k as usize, mid) {
            right = mid
        } else {
            left = mid + 1;
        }
    }
    left
}

/// f[i][j] 表示给前 i 个人分配工作，工作的分配情况为 j 时，完成所有工作的最短时间
pub fn minimum_time_required_dp(jobs: Vec<i32>, k: i32) -> i32 {
    let len = jobs.len();
    let k = k as usize;
    let mut dp = vec![vec![0; 1 << len]; k];
    let mut sum = vec![0; 1 << len];
    for i in 1..1_usize << len {
        let y = i.trailing_zeros();
        sum[i] = sum[i - (1 << y)] + jobs[y as usize];
    }
    for i in 0..1 << len {
        dp[0][i] = sum[i];
    }
    for i in 1..k {
        for j in 0..1 << len {
            let mut min = i32::MAX;
            let mut x = j;
            while x > 0 {
                min = min.min(dp[i - 1][j - x].max(sum[x]));
                x = (x - 1) & j;
            }
            dp[i][j] = min;
        }
    }
    dp[k - 1][(1 << len) - 1]
}

fn main() {
    assert_eq!(minimum_time_required_dp(vec![3, 2, 3], 1), 8);
    assert_eq!(minimum_time_required_dp(vec![1, 2, 4, 7, 8], 2), 11);
    assert_eq!(minimum_time_required_dp(vec![3, 2, 3], 3), 3);
}

