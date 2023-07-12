//! 石子游戏 V

/// 区间dp。dp[i][j] 表示 [i,j] 获得的积分
pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
    let len = stone_value.len();
    let mut dp = vec![vec![0; len]; len];
    let mut ps = vec![0; len + 1];
    for i in 0..len {
        ps[i + 1] = ps[i] + stone_value[i];
    }

    for a in 1..len {
        for i in 0..len - a {
            let j = i + a;
            for k in i..j {
                let left = ps[k + 1] - ps[i];
                let right = ps[j + 1] - ps[k + 1];
                if left >= right {
                    dp[i][j] = dp[i][j].max(right + dp[k + 1][j]);
                }
                if left <= right {
                    dp[i][j] = dp[i][j].max(left + dp[i][k]);
                }
            }
        }
    }
    dp[0][len - 1]
}

/// dp优化，从O(n3) 优化到 O(n2)
/// dp[i][j] = dp[i][k] + sum(i, k) 和 dp[i][j+1] = dp[i][k] + sum(i, k) 这两次的转移计算是重复的。
/// 太妙了这个写法
pub fn stone_game_v2(stone_value: Vec<i32>) -> i32 {
    let len = stone_value.len();
    let mut dp = vec![vec![0; len]; len];
    let mut maxl = vec![vec![0; len]; len]; // 表示 [i, j] 中，dp[i][k] + sum(i,k) 的最大值。
    let mut maxr = vec![vec![0; len]; len];
    for left in (0..len).rev() {
        maxl[left][left] = stone_value[left];
        maxr[left][left] = stone_value[left];
        let mut i = left;  // i表示 [left, right] 中，sum(left..i) <= sum(i..=r). sum(left..=i) > sum(i+1..=r)
        let mut sum = stone_value[left];
        let mut suml = 0;
        for right in left + 1..len {
            sum += stone_value[right];
            while i < right && (suml + stone_value[i]) * 2 <= sum {
                suml += stone_value[i];
                i += 1;
            }
            if left < i {
                dp[left][right] = dp[left][right].max(maxl[left][i - 1]);
            }
            if i < right {
                dp[left][right] = dp[left][right].max(maxr[i + 1][right]);
            }
            if suml * 2 == sum {
                dp[left][right] = dp[left][right].max(maxr[i][right]);
            }
            maxl[left][right] = maxl[left][right - 1].max(sum + dp[left][right]);
            maxr[left][right] = maxr[left + 1][right].max(sum + dp[left][right]);
        }
    }
    dp[0][len - 1]
}

fn main() {
    fn test(func: fn(stone_value: Vec<i32>) -> i32) {
        assert_eq!(func(vec![98, 77, 24, 49, 6, 12, 2, 44, 51, 96]), 330);
        assert_eq!(func(vec![9, 8, 2, 4, 6, 3, 5, 1, 7]), 34);
        assert_eq!(func(vec![6, 2, 3, 4, 5, 5]), 18);
        assert_eq!(func(vec![7, 7, 7, 7, 7, 7, 7]), 28);
        assert_eq!(func(vec![4]), 0);
    }
    test(stone_game_v);
    test(stone_game_v2);
}
