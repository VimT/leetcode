//! 铺瓷砖

/// 完全背包问题：有n种物品和一个容量为mn的背包，每种物品都有无限件可用。第i种物品的体积是ci^2，价值是1。将哪些物品装入背包可使这些物品的体积总和等于背包容量，且价值总和最小。
/// 那么对于本题而言，不妨设m≥n，可以将mn的矩形看成是一个容量为mn的背包，有n种物品（边长为1到n的正方形），每种物品的体积为正方形边长的平方。
/// 可以看出，本题应该是强于上述改写的完全背包问题的（因为还需要考虑如何放置正方形，改写的完全背包问题只需要考虑总面积）。
/// 由于完全背包问题是NP完全问题，故此题不存在多项式时间解法。
/// 在 n,m <= 13 的情况下，可以简化分类：对一个方块，可以横切/竖切/中间切
pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
    fn dfs(mut x: i32, mut y: i32, cache: &mut Vec<Vec<i32>>) -> i32 {
        if x == y { return 1; } else if x == 1 { return y; } else if y == 1 { return x; }
        if x < y { std::mem::swap(&mut x, &mut y); }
        if cache[x as usize][y as usize] < i32::MAX { return cache[x as usize][y as usize]; }
        let mut result = x * y;
        // 竖切
        for i in 1..=x / 2 {
            result = result.min(dfs(i, y, cache) + dfs(x - i, y, cache));
        }
        // 横切
        for i in 1..=y / 2 {
            result = result.min(dfs(x, i, cache) + dfs(x, y - i, cache));
        }
        // 中间有个方块
        for center in 1..x.min(y) {
            for i in 1..x - center {
                for j in 1..y - center {
                    let a = dfs(i, j + center, cache);
                    let b = dfs(j, x - i, cache);
                    let c = dfs(x - i - center, y - j, cache);
                    let d = dfs(y - j - center, i + center, cache);
                    result = result.min(a + b + c + d + 1);
                }
            }
        }
        cache[x as usize][y as usize] = result;
        result
    }
    let side = n.max(m) as usize;
    let mut cache = vec![vec![i32::MAX; side + 1]; side + 1];
    dfs(n, m, &mut cache)
}

/// 打表（狗头）
pub fn tiling_rectangle2(n: i32, m: i32) -> i32 {
    static TABLE: [[i32; 13]; 13] = [
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        [2, 1, 3, 2, 4, 3, 5, 4, 6, 5, 7, 6, 8],
        [3, 3, 1, 4, 4, 2, 5, 5, 3, 6, 6, 4, 7],
        [4, 2, 4, 1, 5, 3, 5, 2, 6, 4, 6, 3, 7],
        [5, 4, 4, 5, 1, 5, 5, 5, 6, 2, 6, 6, 6],
        [6, 3, 2, 3, 5, 1, 5, 4, 3, 4, 6, 2, 6],
        [7, 5, 5, 5, 5, 5, 1, 7, 6, 6, 6, 6, 6],
        [8, 4, 5, 2, 5, 4, 7, 1, 7, 5, 6, 3, 6],
        [9, 6, 3, 6, 6, 3, 6, 7, 1, 6, 7, 4, 7],
        [10, 5, 6, 4, 2, 4, 6, 5, 6, 1, 6, 5, 7],
        [11, 7, 6, 6, 6, 6, 6, 6, 7, 6, 1, 7, 6],
        [12, 6, 4, 3, 6, 2, 6, 3, 4, 5, 7, 1, 7],
        [13, 8, 7, 7, 6, 6, 6, 6, 7, 7, 6, 7, 1],
    ];
    return TABLE[n as usize - 1][m as usize - 1];
}

fn main() {
    fn test(func: fn(n: i32, m: i32) -> i32) {
        assert_eq!(func(2, 3), 3);
        assert_eq!(func(5, 8), 5);
        assert_eq!(func(11, 13), 6);
    }
    test(tiling_rectangle);
    test(tiling_rectangle2);
}
