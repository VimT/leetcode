//! 恰好移动 k 步到达某一位置的方法数目


pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    if (start_pos - end_pos).abs() & 1 != k & 1 {
        return 0;
    }

    fn dfs(cur: i32, end: i32, k: i32, mem: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if k == 0 && cur == end {
            return 1;
        } else if k == 0 { return 0; }
        if let Some(result) = mem[(cur + 1000) as usize][k as usize] {
            return result;
        }
        let result = (dfs(cur + 1, end, k - 1, mem) + dfs(cur - 1, end, k - 1, mem)) % MOD;
        mem[(cur + 1000) as usize][k as usize] = Some(result);
        result
    }
    dfs(start_pos, end_pos, k, &mut vec![vec![None; k as usize + 1]; 3005])
}

/// 数学：从start_pos出发，往正方向走了a步，往负方向走了(k-a)步到达end_pos，答案为 C(k, a)
pub fn number_of_ways2(start_pos: i32, end_pos: i32, k: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let d = (start_pos - end_pos).abs();
    if (d + k) & 1 == 1 || d > k { return 0; }
    let k = k as usize;
    // 模板：递推求组合数
    let mut f = vec![vec![0; k + 1]; k + 1];
    for i in 0..=k {
        f[i][0] = 1;
        for j in 1..=i {
            f[i][j] = (f[i - 1][j] + f[i - 1][j - 1]) % MOD;
        }
    }
    f[k][(d as usize + k) / 2]
}

fn main() {
    fn test(func: fn(start_pos: i32, end_pos: i32, k: i32) -> i32) {
        assert_eq!(func(1, 2, 3), 3);
        assert_eq!(func(2, 5, 10), 0);
    }
    test(number_of_ways);
    test(number_of_ways2);
}
