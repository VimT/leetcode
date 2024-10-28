//! 最大公约数相等的子序列数量

use leetcode::gcd::gcd;

pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mx = nums.iter().max().copied().unwrap() as usize;
    const MOD: i32 = 1e9 as i32 + 7;
    fn dfs(nums: &Vec<i32>, i: usize, x: usize, y: usize, mem: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if i == nums.len() { return (x > 0 && x == y) as i32; }
        if mem[i][x][y] != -1 { return mem[i][x][y]; }
        let mut result = dfs(nums, i + 1, x, y, mem);
        result = (result + dfs(nums, i + 1, gcd(x as i32, nums[i]) as usize, y, mem)) % MOD;
        result = (result + dfs(nums, i + 1, x, gcd(y as i32, nums[i]) as usize, mem)) % MOD;
        mem[i][x][y] = result;
        result
    }

    dfs(&nums, 0, 0, 0, &mut vec![vec![vec![-1; mx + 1]; mx + 1]; n])
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4]), 10);
        assert_eq!(func(vec![10, 20, 30]), 2);
        assert_eq!(func(vec![1, 1, 1, 1]), 50);
    }
    test(subsequence_pair_count);
}
