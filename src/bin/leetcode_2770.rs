//! 达到末尾下标所需的最大跳跃次数

use leetcode::segment_tree::{DynamicSegmentTree, SegmentSpec};

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
    pub enum Max {}
    impl SegmentSpec for Max {
        type ValType = i32;
        type LazyType = i32;
        fn op(&a: &Self::ValType, &b: &Self::ValType) -> Self::ValType { a.max(b) }
        fn identity() -> Self::ValType { i32::MIN }
        fn compose(&f: &Self::LazyType, g: &Self::LazyType) -> Self::LazyType { f.max(*g) }
        fn apply(&f: &Self::LazyType, a: &Self::ValType, _: i64) -> Self::ValType { f.max(*a) }
    }
    let len = nums.len();
    let mut mn = nums[0];
    let mut mx = nums[0];
    for &num in &nums[1..] {
        mn = mn.min(num);
        mx = mx.max(num);
    }
    let mut st = DynamicSegmentTree::<Max>::new(len * 4, mn as i64, mx as i64);
    st.update(nums[0] as i64, nums[0] as i64, &0);
    let target = target as i64;
    for i in 1..len {
        let num = nums[i] as i64;
        let q = st.query(num - target, num + target) + 1;
        st.update(num, num, &q);
        if i == len - 1 {
            return q.max(-1);
        }
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![-492309551, -582749469, 203358841, 981487464, 318778376, 227890401, 89900316], 0), -1);
        assert_eq!(func(vec![1, 0], 0), -1);
        assert_eq!(func(vec![1, 3, 6, 4, 1, 2], 0), -1);
        assert_eq!(func(vec![1, 3, 6, 4, 1, 2], 2), 3);
        assert_eq!(func(vec![1, 3, 6, 4, 1, 2], 3), 5);
    }
    test(maximum_jumps);
    test(maximum_jumps2);
    test(maximum_jumps3);
}
