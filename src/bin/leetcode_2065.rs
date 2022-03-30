//! 最大化一张图中的路径价值

pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
    let len = values.len();
    fn dfs(values: &Vec<i32>, edges: &Vec<Vec<(usize, i32)>>, cur: usize, cur_time: i32, cur_value: i32, result: &mut i32, max_time: i32, vis: &mut Vec<bool>) {
        if cur_time > max_time {
            return;
        }
        if cur == 0 {
            *result = cur_value.max(*result);
        }
        for &(nxt, nxt_time) in &edges[cur] {
            if vis[nxt] {
                dfs(values, edges, nxt, cur_time + nxt_time, cur_value, result, max_time, vis);
            } else {
                vis[nxt] = true;
                dfs(values, edges, nxt, cur_time + nxt_time, cur_value + values[nxt], result, max_time, vis);
                vis[nxt] = false;
            }
        }
    }
    let mut eg = vec![vec![]; len];
    for i in edges {
        eg[i[0] as usize].push((i[1] as usize, i[2]));
        eg[i[1] as usize].push((i[0] as usize, i[2]));
    }
    let mut vis = vec![false; len];
    let mut result = 0;
    vis[0] = true;
    dfs(&values, &eg, 0, 0, values[0], &mut result, max_time, &mut vis);
    result
}

fn main() {
    assert_eq!(maximal_path_quality(vec![0, 32, 10, 43], vec![vec![0, 1, 10], vec![1, 2, 15], vec![0, 3, 10]], 49), 75);
    assert_eq!(maximal_path_quality(vec![5, 10, 15, 20], vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 3, 10]], 30), 25);
    assert_eq!(maximal_path_quality(vec![1, 2, 3, 4], vec![vec![0, 1, 10], vec![1, 2, 11], vec![2, 3, 12], vec![1, 3, 13]], 50), 7);
    assert_eq!(maximal_path_quality(vec![0, 1, 2], vec![vec![1, 2, 10]], 10), 0);
}

