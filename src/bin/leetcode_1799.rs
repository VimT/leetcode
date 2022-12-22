//! N 次操作后的最大分数和

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { return b; }
    gcd(b % a, a)
}

/// 24ms
pub fn max_score(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp = vec![0; 1 << len];
    let mut g = vec![vec![0; len]; len];
    for i in 0..len {
        for j in i + 1..len {
            g[i][j] = gcd(nums[i], nums[j]);
        }
    }
    for s in 1..1usize << len {
        let mut t = s.count_ones();
        if t & 1 == 0 {
            t >>= 1;
            for i in 0..len {
                if s >> i & 1 == 1 {
                    for j in i + 1..len {
                        if s >> j & 1 == 1 {
                            dp[s] = dp[s].max(dp[s ^ (1 << i) ^ (1 << j)] + t as i32 * g[i][j]);
                        }
                    }
                }
            }
        }
    }
    dp[(1 << len) - 1]
}

/// 递归版，96ms
pub fn max_score2(nums: Vec<i32>) -> i32 {
    fn dfs(nums: &Vec<i32>,  status: usize, cnt: i32, cache: &mut Vec<i32>) -> i32 {
        if cnt == (nums.len() / 2) as i32 + 1 { return 0; }
        if cache[status] != -1 {
            return cache[status];
        }
        let mut result = 0;
        for i in 0..nums.len() {
            if status >> i & 1 == 0 {
                for j in i + 1..nums.len() {
                    if status >> j & 1 == 0 {
                        result = result.max(dfs(nums, status | (1 << i) | (1 << j), cnt + 1, cache) + cnt * gcd(nums[i], nums[j]));
                    }
                }
            }
        }
        cache[status] = result;
        result
    }
    let len = nums.len();
    let mut cache = vec![-1; 1 << len];
    dfs(&nums, 0, 1, &mut cache)
}

/// 可以先分组，再计算gcd score（gcd小的排前面），132ms
pub fn max_score3(nums: Vec<i32>) -> i32 {
    fn dfs(nums: &Vec<i32>, seen: i32, g: &Vec<Vec<i32>>, cur: &mut Vec<i32>, result: &mut i32) {
        let len = nums.len();
        if cur.len() == len / 2 {
            let mut data = cur.clone();
            data.sort_unstable();
            *result = (*result).max(data.iter().enumerate().map(|(i, &v)| (i as i32 + 1) * v).sum());
            return;
        }
        let i = (0..len).filter(|&x| seen >> x & 1 == 0).next().unwrap();
        for j in i + 1..len {
            if seen >> j & 1 == 0 {
                cur.push(g[i][j]);
                dfs(nums, seen | (1 << i) | (1 << j), g, cur, result);
                cur.pop();
            }
        }
    }
    let len = nums.len();
    let mut g = vec![vec![0; len]; len];
    for i in 0..len {
        for j in i + 1..len {
            g[i][j] = gcd(nums[i], nums[j]);
        }
    }
    let mut result = 0;
    dfs(&nums, 0, &g, &mut vec![], &mut result);
    result
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![889509, 644676, 621999, 606262, 412720, 678593]), 28);
        assert_eq!(func(vec![1, 2]), 1);
        assert_eq!(func(vec![3, 4, 6, 8]), 11);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6]), 14);
    }
    test(max_score);
    test(max_score2);
    test(max_score3);
}
