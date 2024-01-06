//! 树节点的第 K 个祖先

struct TreeAncestor {
    level: Vec<Vec<i32>>,
}

/// 类似于跳表的思路，第0层是parent，第1层是parent(parent)，第二层是pppp
/// 或者叫 dp倍增法，用 dp[node][level] 也可以表示， dp[node][level] = dp[dp[node][level-1]][level-1]
impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut level = vec![parent];
        let mut last_level = level.last().unwrap();
        let len = n as usize;
        loop {
            let mut this_level = vec![-1; len];
            let mut has = false;
            for i in 0..len {
                this_level[i] = if last_level[i] == -1 || last_level[last_level[i] as usize] == -1 {
                    -1
                } else {
                    has = true;
                    last_level[last_level[i] as usize]
                }
            }
            if !has { break; }
            level.push(this_level);
            last_level = level.last().unwrap();
        }
        Self { level }
    }

    fn get_kth_ancestor(&self, mut node: i32, mut k: i32) -> i32 {
        while k > 0 {
            let level = k.trailing_zeros() as usize;
            if level >= self.level.len() { return -1; }
            node = self.level[level][node as usize];
            if node == -1 { break; }
            k -= 1 << level;
        }
        node
    }
}


fn main() {
    let ta = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);

    assert_eq!(ta.get_kth_ancestor(3, 1), 1);  // 返回 1 ，它是 3 的父节点
    assert_eq!(ta.get_kth_ancestor(5, 2), 0);  // 返回 0 ，它是 5 的祖父节点
    assert_eq!(ta.get_kth_ancestor(6, 3), -1);  // 返回 -1 因为不存在满足要求的祖先节点
}
