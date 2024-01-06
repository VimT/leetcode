//! 移除栅栏得到的正方形田地的最大面积

use std::collections::HashSet;

pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
    fn cal(mut h: Vec<i32>, mx: i32) -> HashSet<i32> {
        h.push(1);
        h.push(mx);
        h.sort_unstable();
        let len = h.len();
        let mut result = HashSet::new();
        for i in 0..len {
            for j in 0..i {
                result.insert(h[i] - h[j]);
            }
        }
        result
    }
    const MOD: i64 = 1e9 as i64 + 7;
    cal(h_fences, m).intersection(&cal(v_fences, n)).max().copied().map(|x| x as i64 * x as i64 % MOD).unwrap_or(-1) as i32
}

fn main() {
    fn test(func: fn(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32) {
        assert_eq!(func(4, 3, vec![2, 3], vec![2]), 4);
        assert_eq!(func(6, 7, vec![2], vec![4]), -1);
    }
    test(maximize_square_area);
}
