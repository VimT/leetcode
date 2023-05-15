//! 使二叉树所有路径值相等的最小代价

/// 自己写的：当前节点要加的 = 当前节点的子树路径最大值 + 到当前路径的和，
pub fn min_increments(_: i32, cost: Vec<i32>) -> i32 {
    fn dfs1(cost: &Vec<i32>, i: usize, max: &mut Vec<i32>) {
        if i * 2 + 1 >= cost.len() {
            max[i] = cost[i];
            return;
        }
        dfs1(cost, i * 2 + 1, max);
        dfs1(cost, i * 2 + 2, max);
        max[i] = max[i * 2 + 1].max(max[i * 2 + 2]) + cost[i];
    }
    fn dfs(cost: &Vec<i32>, max: &Vec<i32>, long: i32, mut cur: i32, i: usize, result: &mut i32) {
        if i >= cost.len() { return; }
        let diff = long - cur - max[i];
        *result += diff;
        cur += cost[i] + diff;
        dfs(cost, max, long, cur, i * 2 + 1, result);
        dfs(cost, max, long, cur, i * 2 + 2, result);
    }
    let mut result = 0;
    let len = cost.len();
    let mut max = vec![0; len];
    dfs1(&cost, 0, &mut max);
    dfs(&cost, &max, max[0], 0, 0, &mut result);
    result
}

/// 别人写的，妙啊
pub fn min_increments2(n: i32, mut cost: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut result = 0;
    for i in (0..n / 2).rev() {
        let (l, r) = (i * 2 + 1, i * 2 + 2);
        result += (cost[l] - cost[r]).abs(); // 两个子节点变成一样的
        cost[i] += cost[l].max(cost[r]); // 子节点的路径和返回给当前节点
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, cost: Vec<i32>) -> i32) {
        assert_eq!(func(7, vec![1, 5, 2, 2, 3, 3, 1]), 6);
        assert_eq!(func(3, vec![5, 3, 3]), 0);
    }
    test(min_increments);
    test(min_increments2);
}
