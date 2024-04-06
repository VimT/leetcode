//! 找出网格的区域平均强度

pub fn result_grid(image: Vec<Vec<i32>>, threshold: i32) -> Vec<Vec<i32>> {
    let m = image.len();
    let n = image[0].len();
    let mut result = vec![vec![0; n]; m];
    let mut cnt = vec![vec![0; n]; m];
    let threshold = threshold as u32;
    for i in 2..m {
        for j in 2..n {
            let mut ok = true;
            // 检查左右相邻的格子
            for x in i - 2..=i {
                if image[x][j - 2].abs_diff(image[x][j - 1]) > threshold || image[x][j - 1].abs_diff(image[x][j]) > threshold {
                    ok = false;
                    break;
                }
            }
            if !ok { continue; }
            // 检查上下相邻的格子
            for y in j - 2..=j {
                if image[i - 2][y].abs_diff(image[i - 1][y]) > threshold || image[i - 1][y].abs_diff(image[i][y]) > threshold {
                    ok = false;
                    break;
                }
            }
            if !ok { continue; }

            // 合法，计算 3*3 网络平均值
            let mut sum = 0;
            for x in i - 2..=i {
                for y in j - 2..=j {
                    sum += image[x][y];
                }
            }
            let avg = sum / 9;
            // 更新 3*3 网络每个格子的值
            for x in i - 2..=i {
                for y in j - 2..=j {
                    result[x][y] += avg;
                    cnt[x][y] += 1;
                }
            }
        }
    }
    for i in 0..m {
        for j in 0..n {
            if cnt[i][j] == 0 {
                result[i][j] = image[i][j];
            } else {
                result[i][j] /= cnt[i][j];
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(image: Vec<Vec<i32>>, threshold: i32) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![1, 1, 4, 1], vec![10, 8, 13, 17], vec![2, 12, 1, 16]], 14), vec![vec![5, 5, 5, 1], vec![5, 5, 5, 17], vec![5, 5, 5, 16]]);
        assert_eq!(func(vec![vec![0, 7, 0], vec![4, 6, 3], vec![2, 4, 5]], 5), vec![vec![0, 7, 0], vec![4, 6, 3], vec![2, 4, 5]]);
        assert_eq!(func(vec![vec![5, 6, 7, 10], vec![8, 9, 10, 10], vec![11, 12, 13, 10]], 3), vec![vec![9, 9, 9, 9], vec![9, 9, 9, 9], vec![9, 9, 9, 9]]);
        assert_eq!(func(vec![vec![10, 20, 30], vec![15, 25, 35], vec![20, 30, 40], vec![25, 35, 45]], 12), vec![vec![25, 25, 25], vec![27, 27, 27], vec![27, 27, 27], vec![30, 30, 30]]);
        assert_eq!(func(vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]], 1), vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]]);
    }
    test(result_grid);
}
