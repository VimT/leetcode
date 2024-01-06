//! 统计全 1 子矩形

pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut dp = vec![vec![0; n]; m];  // 这个块的左边有多少个1（包含自己）
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 1 {
                dp[i][j] = if j > 0 { dp[i][j - 1] } else { 0 } + 1;
            } else {
                dp[i][j] = 0;
            }
        }
    }
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            let mut cur_min = dp[i][j];
            for k in (0..=i).rev() {
                if dp[k][j] == 0 { break; }
                cur_min = cur_min.min(dp[k][j]);
                result += cur_min;
            }
        }
    }
    result
}

/// 单调栈，O(nm)，这个height很巧妙
pub fn num_submat2(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut dp = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 1 {
                dp[i][j] = if j > 0 { dp[i][j - 1] } else { 0 } + 1;
            } else {
                dp[i][j] = 0;
            }
        }
    }
    let mut result = 0;
    for j in 0..n {
        let mut sum = 0;
        let mut q: Vec<(i32, i32)> = vec![];
        for i in 0..m {
            let mut height = 1;
            while !q.is_empty() && q.last().unwrap().0 > dp[i][j] {
                let (x, y) = q.pop().unwrap();
                sum -= y * (x - dp[i][j]);
                height += y;
            }
            sum += dp[i][j];
            result += sum;
            q.push((dp[i][j], height));
        }
    }
    result
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]), 13);
        assert_eq!(func(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]]), 24);
    }
    test(num_submat);
    test(num_submat2);
}
