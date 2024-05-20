//! 到达第 K 级台阶的方案数

use std::collections::HashMap;

pub fn ways_to_reach_stair(k: i32) -> i32 {
    fn dfs(k: usize, i: usize, jump: usize, can_down: bool, mem: &mut HashMap<(usize, usize, bool), i32>) -> i32 {
        if (can_down && i > k + 2) || (!can_down && i > k + 1) { return 0; }
        if let Some(&v) = mem.get(&(i, jump, can_down)) { return v; }
        let mut result = 0;
        if i == k { result += 1; }
        if can_down { result += dfs(k, i - 1, jump, false, mem); }
        result += dfs(k, i + (1 << jump), jump + 1, true, mem);
        mem.insert((i, jump, can_down), result);
        result
    }
    dfs(k as usize, 1, 0, true, &mut HashMap::new())
}

fn main() {
    fn test(func: fn(k: i32) -> i32) {
        assert_eq!(func(0), 2);
        assert_eq!(func(1), 4);
    }
    test(ways_to_reach_stair);
}
