//! 从树中删除边的最小分数

/// dfs时间戳，记录dfs过程中每个节点的入时间和出时间，如果y是以x为根的子树的节点，那么 [iny, outy] 是 [inx, outx]的子区间
pub fn minimum_score(nums: Vec<i32>, mut edges: Vec<Vec<i32>>) -> i32 {
    let len = nums.len();
    let mut g = vec![vec![]; len];
    for edge in &edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }

    let mut xr = vec![0; len];
    let mut in_ = vec![0; len];
    let mut out = vec![0; len];
    fn dfs(g: &Vec<Vec<usize>>, nums: &Vec<i32>, in_: &mut Vec<i32>, out: &mut Vec<i32>, xr: &mut Vec<i32>, x: usize, fa: usize, clock: &mut i32) {
        *clock += 1;
        in_[x] = *clock;
        xr[x] = nums[x];
        for &y in &g[x] {
            if y != fa {
                dfs(g, nums, in_, out, xr, y, x, clock);
                xr[x] ^= xr[y];
            }
        }
        out[x] = *clock;
    }
    let mut clock = 0;

    // 以0为根，计算每个节点的 时间戳，和子树xor
    dfs(&g, &nums, &mut in_, &mut out, &mut xr, 0, 0, &mut clock);
    let is_parent = |x: usize, y: usize| -> bool {
        return in_[x] <= in_[y] && in_[y] <= out[x];
    };
    for edge in &mut edges {
        if !is_parent(edge[0] as usize, edge[1] as usize) {
            edge.swap(0, 1);
        }
    }
    let mut result = i32::MAX;
    let elen = edges.len();
    for i in 0..elen {
        let x1 = edges[i][0] as usize;
        let y1 = edges[i][1] as usize;
        for j in 0..i {
            let x2 = edges[j][0] as usize;
            let y2 = edges[j][1] as usize;
            let x;
            let y;
            let z;

            // y1 是 x2 的父节点（或重合）
            if is_parent(y1, x2) {
                x = xr[y2];
                y = xr[y1] ^ xr[y2];
                z = xr[0] ^ xr[y1];
            } else if is_parent(y2, x1) {
                // y2 是 x1 的父节点（或重合）
                x = xr[y1];
                y = xr[y2] ^ xr[y1];
                z = xr[0] ^ xr[y2];
            } else {
                // 删除的两条边分别属于两颗不相交的子树
                x = xr[y1];
                y = xr[y2];
                z = xr[0] ^ xr[y1] ^ xr[y2];
            }
            result = result.min(x.max(y).max(z) - x.min(y).min(z));
        }
    }

    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![1, 5, 5, 4, 11], vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]]), 9);
        assert_eq!(func(vec![5, 5, 2, 4, 4, 2], vec![vec![0, 1], vec![1, 2], vec![5, 2], vec![4, 3], vec![1, 3]]), 0);
    }
    test(minimum_score);
}
