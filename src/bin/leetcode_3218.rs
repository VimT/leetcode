//! 切蛋糕的最小总开销 I

pub fn minimum_cost(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32 {
    fn dfs(h: &Vec<i32>, v: &Vec<i32>, a: (usize, usize), b: (usize, usize), mem: &mut Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
        if b.0 - a.0 == 1 && b.1 - a.1 == 1 { return 0; }
        if mem[a.0][a.1][b.0][b.1] != -1 {
            return mem[a.0][a.1][b.0][b.1];
        }
        let mut result = i32::MAX;
        for i in a.0 + 1..b.0 {
            result = result.min(h[i - 1] + dfs(h, v, a, (i, b.1), mem) + dfs(h, v, (i, a.1), b, mem));
        }
        for j in a.1 + 1..b.1 {
            result = result.min(v[j - 1] + dfs(h, v, a, (b.0, j), mem) + dfs(h, v, (a.0, j), b, mem));
        }
        mem[a.0][a.1][b.0][b.1] = result;
        result
    }
    let (m, n) = (m as usize, n as usize);
    dfs(&horizontal_cut, &vertical_cut, (0, 0), (m, n), &mut vec![vec![vec![vec![-1; n + 1]; m + 1]; n + 1]; m + 1])
}

fn main() {
    fn test(func: fn(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32) {
        assert_eq!(func(3, 2, vec![1, 3], vec![5]), 13);
        assert_eq!(func(2, 2, vec![7], vec![4]), 15);
    }
    test(minimum_cost);
}
