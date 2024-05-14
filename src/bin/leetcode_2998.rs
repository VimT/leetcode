//! 使 X 和 Y 相等的最少操作次数


pub fn minimum_operations_to_make_equal(x: i32, y: i32) -> i32 {
    // 从x变成y需要多少步
    fn dfs(x: i32, y: i32, cache: &mut Vec<i32>) -> i32 {
        if x == y { return 0; }
        if cache[x as usize] != -1 {
            return cache[x as usize];
        }
        if x < y {
            return y - x;
        }
        let mut result = i32::MAX;
        let m = x % 5;
        result = result.min(dfs((x - m) / 5, y, cache) + m + 1);
        result = result.min(dfs((x - m) / 5 + 1, y, cache) + 5 - m + 1);

        let m = x % 11;
        result = result.min(dfs((x - m) / 11, y, cache) + m + 1);
        result = result.min(dfs((x - m) / 11 + 1, y, cache) + 11 - m + 1);
        result = result.min(dfs(x - 1, y, cache) + 1);
        cache[x as usize] = result;
        result
    }
    let mut cache = vec![-1; 6e5 as usize + 1];
    dfs(x, y, &mut cache)
}

fn main() {
    fn test(func: fn(x: i32, y: i32) -> i32) {
        assert_eq!(func(26, 1), 3);
        assert_eq!(func(54, 2), 4);
        assert_eq!(func(25, 30), 5);
    }
    test(minimum_operations_to_make_equal);
}
