//! 正方形数组的数目


use std::collections::{HashMap, HashSet};

fn is_square_arr(nums: &[i32]) -> bool {
    for win in nums.windows(2) {
        let num = win[0] + win[1];
        let sq = (num as f64).sqrt() as i32;
        if sq * sq != num {
            return false;
        }
    }
    true
}

/// 47. 全排列2
pub fn num_squareful_perms(mut nums: Vec<i32>) -> i32 {
    fn backtrack(nums: &mut Vec<i32>, first: usize, ans: &mut i32, len: usize) {
        if first == len {
            if is_square_arr(nums) {
                *ans += 1;
            }
        }
        let mut set = HashSet::new();
        for i in first..len {
            if set.contains(&nums[i]) { continue; }
            nums.swap(i, first);
            if is_square_arr(&nums[(first as i32 - 1).max(0) as usize..first + 1]) {
                backtrack(nums, first + 1, ans, len);
            }
            nums.swap(i, first);
            set.insert(nums[i]);
        }
    }
    let len = nums.len();
    let mut ans = 0;
    backtrack(&mut nums, 0, &mut ans, len);
    ans
}

/// 构造一张图，包含所有的边 i 到 j ，如果满足 A[i] + A[j]是一个完全平方数。
/// 我们的目标就是求这张图的所有哈密顿路径，即经过图中所有点仅一次的路径。
pub fn num_squareful_perms2(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut cnt = HashMap::new();
    let mut graph = HashMap::new();
    for &num in &nums {
        *cnt.entry(num).or_insert(0i32) += 1;
    }
    let keys: Vec<i32> = cnt.keys().map(|x| *x).collect();
    for &x in &keys {
        for &y in &keys {
            let r = ((x + y) as f64).sqrt() as i32;
            if r * r == x + y {
                graph.entry(x).or_insert(vec![]).push(y);
            }
        }
    }
    fn dfs(cnt: &mut HashMap<i32, i32>, graph: &HashMap<i32, Vec<i32>>, x: i32, todo: i32) -> i32 {
        *cnt.get_mut(&x).unwrap() -= 1;
        let mut result = 1;
        if todo != 0 {
            result = 0;
            if let Some(v) = graph.get(&x) {
                for &y in v {
                    if cnt.get(&y).unwrap_or(&0) != &0 {
                        result += dfs(cnt, graph, y, todo - 1);
                    }
                }
            }
        }
        *cnt.get_mut(&x).unwrap() += 1;
        result
    }
    let mut result = 0;
    for &x in &keys {
        result += dfs(&mut cnt, &graph, x, (len - 1) as i32);
    }
    result
}

/// 因为节点的数量非常少，所以可以使用掩码标记所有已经过点的方式来进行动态规划。
pub fn num_squareful_perms3(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut cnt = HashMap::new();
    let mut graph = vec![vec![]; len];
    let mut dp = vec![vec![-1; 1 << len]; len];
    for &num in &nums {
        *cnt.entry(num).or_insert(0i32) += 1;
    }
    for i in 0..len {
        for j in i + 1..len {
            let r = ((nums[i] + nums[j]) as f64).sqrt() as i32;
            if r * r == nums[i] + nums[j] {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }
    let mut factorial = vec![0; 20];
    factorial[0] = 1;
    for i in 1..20 {
        factorial[i] = i * factorial[i - 1];
    }
    fn dfs(graph: &Vec<Vec<usize>>, dp: &mut Vec<Vec<i32>>, node: usize, visited: usize) -> i32 {
        if visited == (1 << dp.len()) - 1 {
            return 1;
        }
        if dp[node][visited] != -1 {
            return dp[node][visited];
        }
        let mut result = 0;
        for &nei in &graph[node] {
            if (visited >> nei) & 1 == 0 {
                result += dfs(graph, dp, nei, visited | (1 << nei));
            }
        }
        dp[node][visited] = result;
        return result;
    }
    let mut result = 0;
    for i in 0..len {
        result += dfs(&graph, &mut dp, i, 1 << i);
    }
    for &v in cnt.values() {
        result /= factorial[v as usize] as i32;
    }
    result
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 17, 8]), 2);
        assert_eq!(func(vec![2, 2, 2]), 1);
    }
    test(num_squareful_perms);
    test(num_squareful_perms2);
    test(num_squareful_perms3);
}
