//! 特别的排列

pub fn special_perm(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut g = vec![vec![]; len];
    for i in 0..len {
        for j in 0..len {
            if nums[i] % nums[j] == 0 {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }
    const MOD: i32 = 1e9 as i32 + 7;
    fn dfs(g: &Vec<Vec<usize>>, u: usize, vis: usize, mem: &mut Vec<Vec<i32>>) -> i32 {
        if mem[u][vis] != -1 {
            return mem[u][vis];
        }
        let mut result = 0;
        for &v in &g[u] {
            if vis >> v & 1 == 0 {
                result += dfs(g, v, vis | 1 << v, mem);
                result %= MOD;
            }
        }
        mem[u][vis] = result;
        result
    }
    let mut cache = vec![vec![-1; 1 << len]; len];
    for i in 0..len {
        cache[i][(1 << len) - 1] = 1;
    }
    let mut result = 0;
    for i in 0..len {
        result += dfs(&g, i, 1 << i, &mut cache);
        result %= MOD;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![31, 93]), 2);
        assert_eq!(func(vec![2, 3, 6]), 2);
        assert_eq!(func(vec![1, 4, 3]), 2);
    }
    test(special_perm);
}
