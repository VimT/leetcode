pub fn get_max_matrix(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = vec![];
    let m = matrix.len();
    let n = matrix[0].len();
    let mut sumcol = vec![vec![0; n]; m + 1];
    for i in 1..=m {
        for j in 0..n {
            sumcol[i][j] = sumcol[i - 1][j] + matrix[i - 1][j];
        }
    }
    let mut max = i32::min_value();
    for i in 0..m {
        for j in i + 1..=m {
            let mut row = vec![0; n];
            for x in 0..n {
                row[x] = sumcol[j][x] - sumcol[i][x];
            }

            let mut prev = row[0];
            let mut row_max = row[0];
            let mut left = 0;
            let mut real_left = 0;
            let mut right = 0;
            for x in 1..n {
                if prev > 0 {
                    prev += row[x];
                } else {
                    prev = row[x];
                    left = x;
                }
                if prev > row_max {
                    row_max = prev;
                    real_left = left;
                    right = x;
                }
            }
            if row_max > max {
                max = row_max;
                ans = vec![i as i32, real_left as i32, (j - 1) as i32, right as i32];
            }
        }
    }
    ans
}

fn main() {
    println!("{:?}", get_max_matrix(vec![vec![-4, -5]]));
    println!("{:?}", get_max_matrix(vec![vec![-1, 0], vec![0, -1]]));
}
