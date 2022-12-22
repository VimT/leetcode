//! 堆叠长方体的最大高度

/// 排序+贪心+暴力
pub fn max_height(mut cuboids: Vec<Vec<i32>>) -> i32 {
    for cuboid in &mut cuboids {
        cuboid.sort_unstable();
    }
    cuboids.sort_unstable();
    cuboids.reverse();
    let len = cuboids.len();
    let mut dp = vec![0; len];
    for i in 0..len {
        for j in 0..i {
            if cuboids[j][1] >= cuboids[i][1] && cuboids[j][2] >= cuboids[i][2] {
                dp[i] = dp[i].max(dp[j]);
            }
        }
        dp[i] += cuboids[i][2];
    }
    dp.into_iter().max().unwrap()
}

fn main() {
    fn test(func: fn(cuboids: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![50, 45, 20], vec![95, 37, 53], vec![45, 23, 12]]), 190);
        assert_eq!(func(vec![vec![38, 25, 45], vec![76, 35, 3]]), 76);
        assert_eq!(func(vec![vec![7, 11, 17], vec![7, 17, 11], vec![11, 7, 17], vec![11, 17, 7], vec![17, 7, 11], vec![17, 11, 7]]), 102);
    }
    test(max_height);
}
