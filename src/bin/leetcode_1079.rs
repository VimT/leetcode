//! 活字印刷

/// 类似第 90 题 （子集 II）
/// 递归终止条件是：所有的字符都使用完毕。递归终止条件隐含在递归方法中；
pub fn num_tile_possibilities(tiles: String) -> i32 {
    let mut cnt = [0; 26];
    for &ch in tiles.as_bytes() {
        cnt[(ch - b'A') as usize] += 1;
    }
    fn dfs(cnt: &mut [i32; 26]) -> i32 {
        let mut result = 0;
        for i in 0..26 {
            if cnt[i] == 0 { continue; }
            result += 1;
            cnt[i] -= 1;
            result += dfs(cnt);
            cnt[i] += 1;
        }
        result
    }
    dfs(&mut cnt)
}

fn main() {
    assert_eq!(num_tile_possibilities(String::from("AAB")), 8);
    assert_eq!(num_tile_possibilities(String::from("AAABBC")), 188);
    assert_eq!(num_tile_possibilities(String::from("V")), 1);
}
