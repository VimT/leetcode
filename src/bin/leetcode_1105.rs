//! 填充书架

/// dp[i] 表示放置前 i 本书所需要的书架最小高度，初始值 dp[0] = 0，其他为最大值 1000*1000。
/// 遍历每一本书，把当前这本书作为书架最后一层的最后一本书，将这本书之前的书向后调整，看看是否可以减少之前的书架高度。
/// 状态转移方程为 dp[i] = min(dp[i] , dp[j - 1] + h) 其中 j 表示最后一层所能容下书籍的索引，h 表示最后一层最大高度。
pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let len = books.len();
    let mut dp = vec![i32::MAX; len + 1];
    dp[0] = 0;
    for i in 1..=len {
        let mut width = 0;
        let mut h = 0;
        for j in (1..=i).rev() {
            width += books[j - 1][0];
            if width > shelf_width { break; }
            h = h.max(books[j - 1][1]);
            dp[i] = dp[i].min(dp[j - 1] + h);
        }
    }
    dp[len]
}

fn main() {
    fn test(func: fn(books: Vec<Vec<i32>>, shelf_width: i32) -> i32) {
        assert_eq!(func(vec![vec![1, 1], vec![2, 3], vec![2, 3], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 2]], 4), 6);
        assert_eq!(func(vec![vec![1, 3], vec![2, 4], vec![3, 2]], 6), 4);
    }
    test(min_height_shelves);
}
