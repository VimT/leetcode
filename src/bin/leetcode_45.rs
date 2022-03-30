//! 跳跃游戏 II


/// 笨办法模拟，dfs
/// 超时
pub fn jump_dfs(nums: Vec<i32>) -> i32 {
    fn dfs(nums: &Vec<i32>, idx: usize, current: usize, ans: &mut usize) {
        if nums[idx] as usize >= nums.len() - idx - 1 {
            if current + 1 < *ans { *ans = current + 1; }
            return;
        }
        if nums[idx] == 0 { return; }
        if current >= *ans { return; }
        for i in (1..=nums[idx] as usize).rev() {
            dfs(nums, idx + i, current + 1, ans);
            if *ans - current == 1 { break; }
        }
    }
    if nums.len() <= 1 { return 0; }
    let mut ans = nums.len();
    dfs(&nums, 0, 0, &mut ans);
    ans as i32
}

/// 标准解法。 既然一定有答案。那找最短跳就可以
/// 每次跳跃 的跳跃范围中，找最大的跳法。
pub fn jump_stand(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut start = 0;
    let mut end = 1;
    while end < nums.len() {
        let mut max_pos = 0;
        for i in start..end {
            max_pos = max_pos.max(i + nums[i] as usize);
        }
        start = end;
        end = max_pos + 1;
        ans += 1;
    }
    ans
}

/// 优化
pub fn jump_optimise(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut end = 0;
    let mut max_pos = 0;
    for i in 0..nums.len() - 1 {
        max_pos = (nums[i] + i as i32).max(max_pos);
        if i == end {
            end = max_pos as usize;
            ans += 1;
        }
    }
    ans
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(func(vec![2, 3, 0, 1, 4]), 2);
    }
    test(jump_dfs);
    test(jump_stand);
    test(jump_optimise);
}
