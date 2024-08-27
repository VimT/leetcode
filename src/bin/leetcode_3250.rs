//! 单调数组对的数目 I

pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    // 当前正在验证数字 nums[i] ，上一个递增到 a， 上一个递减到 b
    // x1 >= a, x2 <= b, x1 + x2 = nums[i]
    fn dfs(nums: &Vec<i32>, i: usize, a: i32, mem: &mut Vec<Vec<i32>>) -> i32 {
        if i == nums.len() { return 1; }
        if mem[i][a as usize] != -1 {
            return mem[i][a as usize];
        }
        let mut result = 0;
        for x1 in a.max(a + nums[i] - if i > 0 { nums[i - 1] } else { 50 })..=nums[i] {
            result = (result + dfs(nums, i + 1, x1, mem)) % MOD;
        }
        mem[i][a as usize] = result;
        result
    }
    dfs(&nums, 0, 0, &mut vec![vec![-1; 51]; nums.len()])
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 3, 2]), 4);
        assert_eq!(func(vec![5, 5, 5, 5]), 126);
    }
    test(count_of_pairs);
}
