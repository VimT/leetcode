//! 所有可能的路径

pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn dfs(graph: &Vec<Vec<i32>>, num: usize, cur: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if num == graph.len() - 1 {
            result.push(cur.clone());
            return;
        }
        for &nxt in &graph[num] {
            cur.push(nxt);
            dfs(graph, nxt as usize, cur, result);
            cur.pop();
        }
    }
    let mut cur = vec![0];
    let mut result = vec![];
    dfs(&graph, 0, &mut cur, &mut result);
    result
}

fn main() {
    assert_eq!(all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]), vec![vec![0, 1, 3], vec![0, 2, 3]]);
    assert_eq!(all_paths_source_target(vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]]), vec![vec![0, 4], vec![0, 3, 4], vec![0, 1, 3, 4], vec![0, 1, 2, 3, 4], vec![0, 1, 4]]);
    assert_eq!(all_paths_source_target(vec![vec![1], vec![]]), vec![vec![0, 1]]);
    assert_eq!(all_paths_source_target(vec![vec![1, 2, 3], vec![2], vec![3], vec![]]), vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]]);
    assert_eq!(all_paths_source_target(vec![vec![1, 3], vec![2], vec![3], vec![]]), vec![vec![0, 1, 2, 3], vec![0, 3]]);
}
