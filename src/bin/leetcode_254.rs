//! 因子的组合

/// 回溯
pub fn get_factors(n: i32) -> Vec<Vec<i32>> {
    fn dfs(n: i32, mut start: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        while start * start <= n {
            if n % start == 0 {
                result.push(vec![start, n / start]);
                for mut tmp in dfs(n / start, start) {
                    tmp.insert(0, start);
                    result.push(tmp);
                }
            }
            start += 1;
        }
        result
    }
    dfs(n, 2)
}

fn main() {
    fn test(func: fn(n: i32) -> Vec<Vec<i32>>) {
        assert_eq!(func(1).is_empty(), true);
        assert_eq!(func(12), vec![vec![2, 6], vec![2, 2, 3], vec![3, 4]]);
        assert_eq!(func(37).is_empty(), true);
    }
    test(get_factors);
}
