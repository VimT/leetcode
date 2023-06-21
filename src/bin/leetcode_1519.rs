//! 子树中标签相同的节点数

pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    fn dfs(g: &Vec<Vec<usize>>, u: usize, fa: usize, labels: &[u8], result: &mut Vec<i32>) -> [i32; 26] {
        let mut this = [0; 26];
        for &v in &g[u] {
            if v != fa {
                let child = dfs(g, v, u, labels, result);
                for i in 0..26 {
                    this[i] += child[i];
                }
            }
        }

        this[(labels[u] - b'a') as usize] += 1;
        result[u] = this[(labels[u] - b'a') as usize];
        return this;
    }
    let mut result = vec![0; n];
    dfs(&g, 0, n, labels.as_bytes(), &mut result);
    result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32>) {
        assert_eq!(func(7, vec![vec![0, 1], vec![0, 2], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 6]], String::from("abaedcd")), vec![2, 1, 1, 1, 1, 1, 1]);
        assert_eq!(func(4, vec![vec![0, 1], vec![1, 2], vec![0, 3]], String::from("bbbb")), vec![4, 2, 1, 1]);
        assert_eq!(func(5, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![0, 4]], String::from("aabab")), vec![3, 2, 1, 1, 1]);
    }
    test(count_sub_trees);
}
