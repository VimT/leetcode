//! 吃掉 N 个橘子的最少天数

use std::collections::HashMap;

pub fn min_days(n: i32) -> i32 {
    fn dfs(n: i32, mem: &mut HashMap<i32, i32>) -> i32 {
        if n < 3 { return n; }
        if let Some(&result) = mem.get(&n) {
            return result;
        }
        let result = 1 + (dfs(n / 2, mem) + n % 2).min(dfs(n / 3, mem) + n % 3);
        mem.insert(n, result);
        result
    }
    dfs(n, &mut HashMap::new())
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(10), 4);
        assert_eq!(func(6), 3);
        assert_eq!(func(1), 1);
        assert_eq!(func(56), 6);
    }
    test(min_days);
}
