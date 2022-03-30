//! 旋转图像

/// 两次翻转
/// 先沿右上 - 左下的对角线翻转（270° +270°+ 一次镜像），再沿水平中线左右翻转（-180° +−180°+ 一次镜像），可以实现顺时针 9090 度的旋转效果
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    if n == 0 { return; }
    for i in 0..n {
        for j in i + 1..n {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }
    for i in 0..n {
        for j in 0..n / 2 {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[i][n - j - 1];
            matrix[i][n - j - 1] = tmp;
        }
    }
}


/// 自外向内顺时针旋转
pub fn rotate_range(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    if n == 0 { return; }
    for times in 0..=n / 2 {
        let len = n - times * 2;
        for i in 0..len - 1 {
            let tmp = matrix[times][times + i];
            matrix[times][times + i] = matrix[times + len - i - 1][times];
            matrix[times + len - i - 1][times] = matrix[times + len - 1][times + len - i - 1];
            matrix[times + len - 1][times + len - i - 1] = matrix[times + i][times + len - 1];
            matrix[times + i][times + len - 1] = tmp;
            println!("({},{}), ({},{}), ({},{}), ({},{})", times, times + i, times + len - i - 1, times, times + len - 1, times + len - i - 1, times + i, times + len - 1);
        }
    }
}


fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate(&mut matrix);
    println!("{:?}", matrix);
}
