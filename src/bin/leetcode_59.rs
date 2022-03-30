//! 螺旋矩阵 II

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    if n == 1 { return vec![vec![1]]; }
    let mut ans = vec![vec![0; n]; n];
    let mut r1 = 0;
    let mut r2 = n - 1;
    let mut count = 1;
    while r1 <= r2 && r1 <= r2 {
        for c in r1..=r2 {
            ans[r1][c] = count;
            count += 1;
        }
        for r in r1 + 1..=r2 {
            ans[r][r2] = count;
            count += 1;
        }
        if r1 < r2 && r1 < r2 {
            for c in (r1 + 1..=r2 - 1).rev() {
                ans[r2][c] = count;
                count += 1;
            }
            for r in (r1 + 1..=r2).rev() {
                ans[r][r1] = count;
                count += 1;
            }
        }
        r1 += 1;
        r2 -= 1;
    }
    ans
}

fn main() {
    for i in 1..10 {
        println!("{:?}", generate_matrix(i));
    }
}

