//! 相同分数的最大操作数目 II

pub fn max_operations(nums: Vec<i32>) -> i32 {
    fn dfs(nums: &Vec<i32>, left: usize, right: usize, target: i32, mem: &mut Vec<Vec<i32>>) -> i32 {
        if right - left < 2 { return 0; }
        if mem[left][right] != -1 { return mem[left][right]; }
        let mut result = 0;
        if nums[left] + nums[right - 1] == target { result = result.max(1 + dfs(nums, left + 1, right - 1, target, mem)); }
        if nums[left] + nums[left + 1] == target { result = result.max(1 + dfs(nums, left + 2, right, target, mem)); }
        if nums[right - 1] + nums[right - 2] == target { result = result.max(1 + dfs(nums, left, right - 2, target, mem)); }
        mem[left][right] = result;
        result
    }
    let len = nums.len();
    let result1 = dfs(&nums, 2, len, nums[0] + nums[1], &mut vec![vec![-1; len + 1]; len + 1]);
    let result2 = dfs(&nums, 0, len - 2, nums[len - 1] + nums[len - 2], &mut vec![vec![-1; len + 1]; len + 1]);
    let result3 = dfs(&nums, 1, len - 1, nums[0] + nums[len - 1], &mut vec![vec![-1; len + 1]; len + 1]);
    result1.max(result2).max(result3) + 1
}

pub fn max_operations2(nums: Vec<i32>) -> i32 {
    fn test(nums: &Vec<i32>, target: i32, start: usize) -> usize {
        let n = nums.len();
        let mut dp1 = vec![0; n + 1];
        let mut dp2 = vec![0; n + 1];
        dp1[start] = 1;
        for i in 1..(n / 2) {
            let curr_len = n - i * 2;
            if curr_len < 2 { break; }
            let mut has_new = false;
            for j in 0..=(n - curr_len) {
                if dp1[j] < i { continue; }
                if nums[j] + nums[j + 1] == target {
                    dp2[j + 2] = i + 1;
                    has_new = true;
                }
                if nums[j] + nums[j + curr_len - 1] == target {
                    dp2[j + 1] = i + 1;
                    has_new = true;
                }
                if nums[j + curr_len - 1] + nums[j + curr_len - 2] == target {
                    dp2[j] = i + 1;
                    has_new = true;
                }
            }
            let tmp = dp1;
            dp1 = dp2;
            dp2 = tmp;
            if !has_new { break; }
        }
        dp1.into_iter().max().unwrap().max(dp2.into_iter().max().unwrap())
    }

    let len = nums.len();
    let result1 = test(&nums, nums[0] + nums[1], 2);
    let result2 = test(&nums, nums[0] + nums[len - 1], 1);
    let result3 = test(&nums, nums[len - 1] + nums[len - 2], 0);
    result1.max(result2).max(result3) as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 2, 1, 2, 3, 4]), 3);
        assert_eq!(func(vec![3, 2, 6, 1, 4]), 2);
    }
    test(max_operations);
    test(max_operations2);
}
