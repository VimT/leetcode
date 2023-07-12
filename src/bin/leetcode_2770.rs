//! 达到末尾下标所需的最大跳跃次数

pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
    fn dfs(nums: &Vec<i32>, i: usize, target: i64, mem: &mut Vec<i32>) -> i32 {
        if i == nums.len() - 1 { return 0; }
        if mem[i] != -2 { return mem[i]; }
        let mut result = -1;
        for j in i + 1..nums.len() {
            if (nums[j] as i64 - nums[i] as i64).abs() <= target {
                let sub = dfs(nums, j, target, mem);
                if sub != -1 { result = result.max(1 + sub); }
            }
        }
        mem[i] = result;
        result
    }
    dfs(&nums, 0, target as i64, &mut vec![-2; nums.len()])
}

pub fn maximum_jumps2(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    let mut dp = vec![-1; len];
    dp[0] = 0;
    for i in 0..len {
        if dp[i] == -1 { continue; }
        for j in i + 1..len {
            if (nums[j] - nums[i]).abs() <= target {
                dp[j] = dp[j].max(dp[i] + 1);
            }
        }
    }
    dp[len - 1]
}

/// 可以优化成 O(nlogU) 时间复杂度 （实际运行时间不如上面的）
/// -target <= nums[j] - nums[i] <= target  ==> nums[i] - target <= nums[j] <= nums[i] + target
pub fn maximum_jumps3(nums: Vec<i32>, target: i32) -> i32 {
    struct Node {
        val: i32,
        l: usize,
        r: usize,
    }
    impl Node {
        fn new() -> Self {
            Self { val: i32::MIN, l: 0, r: 0 }
        }
    }
    // 动态开点线段树
    struct SegmentTree {
        tree: Vec<Node>,
    }
    impl SegmentTree {
        fn push_down(&mut self, p: usize) {
            if self.tree[p].l == 0 {
                self.tree.push(Node::new());
                self.tree[p].l = self.tree.len() - 1;
            }
            if self.tree[p].r == 0 {
                self.tree.push(Node::new());
                self.tree[p].r = self.tree.len() - 1;
            }
        }
        fn query(&mut self, l: i64, r: i64, s: i64, t: i64, p: usize) -> i32 {
            if s >= l && t <= r {
                return self.tree[p].val;
            }
            self.push_down(p);
            let mid = (s + t) >> 1;  // !! 大坑：在s,t包含负数的的时候，必须使用右移而不是除法。-1 / 2 == 0, -1 >> 1 == -1
            let mut result = i32::MIN;
            if l <= mid { result = result.max(self.query(l, r, s, mid, self.tree[p].l)); }
            if r > mid { result = result.max(self.query(l, r, mid + 1, t, self.tree[p].r)); }
            result
        }
        fn update(&mut self, x: i64, s: i64, t: i64, p: usize, val: i32) {
            if x == s && x == t {
                self.tree[p].val = val;
                return;
            }
            self.push_down(p);
            let mid = (s + t) >> 1;
            if x <= mid {
                self.update(x, s, mid, self.tree[p].l, val);
            } else {
                self.update(x, mid + 1, t, self.tree[p].r, val);
            }
            self.tree[p].val = self.tree[self.tree[p].l].val.max(self.tree[self.tree[p].r].val);
        }
        fn new(n: usize) -> Self {
            let mut tree: Vec<Node> = Vec::with_capacity(n);
            tree.push(Node::new());
            tree.push(Node::new());
            Self { tree }
        }
    }
    let len = nums.len();
    let mut st = SegmentTree::new(len * 4);
    let mut mn = nums[0];
    let mut mx = nums[0];
    for &num in &nums[1..] {
        mn = mn.min(num);
        mx = mx.max(num);
    }
    st.update(nums[0] as i64, mn as i64, mx as i64, 1, 0);
    let target = target as i64;
    for i in 1..len {
        let num = nums[i] as i64;
        let q = st.query(num - target, num + target, mn as i64, mx as i64, 1) + 1;
        st.update(num, mn as i64, mx as i64, 1, q);
        if i == len - 1 {
            return q.max(-1);
        }
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![1, 0], 0), -1);
        assert_eq!(func(vec![1, 3, 6, 4, 1, 2], 0), -1);
        assert_eq!(func(vec![1, 3, 6, 4, 1, 2], 2), 3);
        assert_eq!(func(vec![1, 3, 6, 4, 1, 2], 3), 5);
    }
    test(maximum_jumps);
    test(maximum_jumps2);
    test(maximum_jumps3);
}
