//! 包含所有 1 的最小矩形面积 II

pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut result = i32::MAX;

    let mut prefix_sum = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            prefix_sum[i][j] = grid[i - 1][j - 1]
                + prefix_sum[i - 1][j]
                + prefix_sum[i][j - 1]
                - prefix_sum[i - 1][j - 1];
        }
    }

    if prefix_sum[m][n] == 3 {
        return 3;
    }
    if prefix_sum[m][n] == (m * n) as i32 {
        return prefix_sum[m][n];
    }

    // 使用前缀和数组来计算任意矩形内1的数量
    let sum = |x1: usize, y1: usize, x2: usize, y2: usize| -> i32 {
        let result = prefix_sum[x2 + 1][y2 + 1]
            - prefix_sum[x1][y2 + 1]
            - prefix_sum[x2 + 1][y1]
            + prefix_sum[x1][y1];
        result
    };


    // 计算 (x1, y1) -> (x2, y2) 这个区域内能包含所有1的最小的矩形的面积
    let min_rectangle = |mut x1, mut y1, mut x2, mut y2| -> i32 {
        while x1 < x2 && sum(x1, y1, x1, y2) == 0 {
            x1 += 1;
        }
        while x1 < x2 && sum(x2, y1, x2, y2) == 0 {
            x2 -= 1;
        }
        while y1 < y2 && sum(x1, y1, x2, y1) == 0 {
            y1 += 1;
        }
        while y1 < y2 && sum(x1, y2, x2, y2) == 0 {
            y2 -= 1;
        }
        (x2 - x1 + 1) as i32 * (y2 - y1 + 1) as i32
    };

    // 都横向切分，枚举中间两个切割点
    for i in 0..m - 2 {
        for j in i + 1..m - 1 {
            if sum(0, 0, i, n - 1) == 0 || sum(i + 1, 0, j, n - 1) == 0 || sum(j + 1, 0, m - 1, n - 1) == 0 {
                continue;
            }
            let area1 = min_rectangle(0, 0, i, n - 1);
            let area2 = min_rectangle(i + 1, 0, j, n - 1);
            let area3 = min_rectangle(j + 1, 0, m - 1, n - 1);
            result = result.min(area1 + area2 + area3);
        }
    }

    // 都纵向切分，枚举中间两个切割点
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            if sum(0, 0, m - 1, i) == 0 || sum(0, i + 1, m - 1, j) == 0 || sum(0, j + 1, m - 1, n - 1) == 0 {
                continue;
            }
            let area1 = min_rectangle(0, 0, m - 1, i);
            let area2 = min_rectangle(0, i + 1, m - 1, j);
            let area3 = min_rectangle(0, j + 1, m - 1, n - 1);
            result = result.min(area1 + area2 + area3);
        }
    }

    // 一个上面的横向切分，两个纵向切分
    for i in 0..m-1 {
        for j in 0..n - 1 {
            if sum(0, 0, i, n - 1) == 0 || sum(i + 1, 0, m - 1, j) == 0 || sum(i + 1, j + 1, m - 1, n - 1) == 0 {
                continue;
            }
            let area1 = min_rectangle(0, 0, i, n - 1);
            let area2 = min_rectangle(i + 1, 0, m - 1, j);
            let area3 = min_rectangle(i + 1, j + 1, m - 1, n - 1);
            result = result.min(area1 + area2 + area3);
        }
    }

    // 一个下面的横向切分，两个纵向切分
    for i in 1..m {
        for j in 0..n - 1 {
            if sum(i, 0, m - 1, n - 1) == 0 || sum(0, 0, i - 1, j) == 0 || sum(0, j + 1, i - 1, n - 1) == 0 {
                continue;
            }
            let area1 = min_rectangle(i, 0, m - 1, n - 1);
            let area2 = min_rectangle(0, 0, i - 1, j);
            let area3 = min_rectangle(0, j + 1, i - 1, n - 1);
            result = result.min(area1 + area2 + area3);
        }
    }

    // 一个左边的纵向切分，两个横向切分
    for j in 0..n - 1 {
        for i in 0..m - 1 {
            if sum(0, 0, m - 1, j) == 0 || sum(0, j + 1, i, n - 1) == 0 || sum(i + 1, j + 1, m - 1, n - 1) == 0 {
                continue;
            }
            let area1 = min_rectangle(0, 0, m - 1, j);
            let area2 = min_rectangle(0, j + 1, i, n - 1);
            let area3 = min_rectangle(i + 1, j + 1, m - 1, n - 1);
            result = result.min(area1 + area2 + area3);
        }
    }

    // 一个右边的纵向切分，两个横向切分
    for j in 1..n {
        for i in 0..m - 1 {
            if sum(0, j, m - 1, n - 1) == 0 || sum(0, 0, i, j - 1) == 0 || sum(i + 1, 0, m - 1, j - 1) == 0 {
                continue;
            }
            let area1 = min_rectangle(0, j, m - 1, n - 1);
            let area2 = min_rectangle(0, 0, i, j - 1);
            let area3 = min_rectangle(i + 1, 0, m - 1, j - 1);
            result = result.min(area1 + area2 + area3);
        }
    }

    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![1, 1, 0, 1, 1], vec![0, 0, 0, 0, 0], vec![0, 1, 1, 1, 1]]), 8);
        assert_eq!(func(vec![vec![1, 1, 0, 1], vec![0, 0, 0, 1], vec![1, 1, 0, 1]]), 7);
        assert_eq!(func(vec![vec![0, 1], vec![1, 1]]), 3);
        assert_eq!(func(vec![vec![1, 0, 1, 0], vec![0, 1, 0, 1]]), 5);
        assert_eq!(func(vec![vec![1, 0, 1], vec![1, 1, 1]]), 5);
    }
    test(minimum_sum);
}
