//! 元素和小于等于阈值的正方形的最大边长

pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut presum = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            presum[i][j] = mat[i - 1][j - 1] + presum[i - 1][j] + presum[i][j - 1] - presum[i - 1][j - 1];
        }
    }
    let mut left = 0;
    let mut right = m.min(n) + 1;
    // 二分查找：结果是 left-1 -> left写法： mid = (..+1)/2, left=mid,right=mid-1
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut ok = false;
        for i in mid..=m {
            for j in mid..=n {
                if presum[i][j] + presum[i - mid][j - mid] - presum[i][j - mid] - presum[i - mid][j] <= threshold {
                    ok = true;
                    break;
                }
            }
        }
        if ok {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left as i32
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>, threshold: i32) -> i32) {
        assert_eq!(func(vec![vec![18, 70], vec![61, 1], vec![25, 85], vec![14, 40], vec![11, 96], vec![97, 96], vec![63, 45]], 40184), 2);
        assert_eq!(func(vec![vec![2, 2, 2, 2, 2], vec![2, 2, 2, 2, 2], vec![2, 2, 2, 2, 2], vec![2, 2, 2, 2, 2], vec![2, 2, 2, 2, 2]], 1), 0);
        assert_eq!(func(vec![vec![1, 1, 3, 2, 4, 3, 2], vec![1, 1, 3, 2, 4, 3, 2], vec![1, 1, 3, 2, 4, 3, 2]], 4), 2);
    }
    test(max_side_length);
}
