//! 将子数组重新排序得到同一个二叉搜索树的方案数


pub fn num_of_ways(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let len = nums.len();
    let mut left = vec![0; len + 1];
    let mut right = vec![0; len + 1];
    let mut size = vec![1; len + 1];
    size[0] = 0;

    let root = nums[0] as usize;
    for &num in &nums[1..] {
        let num = num as usize;
        let mut parent = root;
        loop {
            size[parent] += 1;
            let sub = if num < parent { &mut left[parent] } else { &mut right[parent] };
            if *sub == 0 {
                *sub = num;
                break;
            }
            parent = *sub;
        }
    }

    // 预计算组合数
    let mut c = vec![vec![0; len]; len];
    c[0][0] = 1;
    for i in 1..len {
        c[i][0] = 1;
        for j in 1..len {
            c[i][j] = (c[i - 1][j - 1] + c[i - 1][j]) % MOD;
        }
    }

    struct DFS {
        c: Vec<Vec<i64>>,
        left: Vec<usize>,
        right: Vec<usize>,
        size: Vec<usize>,
    }
    impl DFS {
        // 以当前节点为根节点，生成的排列个数 f(u) = C[root_size-1][left_size] * f(left) * f(right)
        fn dfs(&mut self, u: usize) -> i64 {
            if u == 0 { return 1; }
            let ll = self.dfs(self.left[u]);
            let rr = self.dfs(self.right[u]);
            let lsize = self.size[self.left[u]];
            let rsize = self.size[self.right[u]];
            return self.c[lsize + rsize][lsize] * ll % MOD * rr % MOD;
        }
    }

    let mut dfs = DFS { c, left, right, size };
    ((dfs.dfs(root) + MOD - 1) % MOD) as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1]), 0);
        assert_eq!(func(vec![2, 1, 3]), 1);
        assert_eq!(func(vec![3, 4, 5, 1, 2]), 5);
        assert_eq!(func(vec![1, 2, 3]), 0);
    }
    test(num_of_ways);
}
