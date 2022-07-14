//! 二维区域和检索 - 可变

/// 二维树状数组
struct NumMatrix {
    bit: Vec<Vec<i32>>,
}


impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut s = Self { bit: vec![vec![0; n + 1]; m + 1] };
        for i in 0..m {
            for j in 0..n {
                s.update(i as i32, j as i32, matrix[i][j]);
            }
        }
        s
    }

    fn update(&mut self, row: i32, col: i32, mut val: i32) {
        val -= self.sum_region(row, col, row, col); // 计算增量
        let mut i = row + 1;
        while i < self.bit.len() as i32 {
            let mut j = col + 1;
            while j < self.bit[0].len() as i32 {
                self.bit[i as usize][j as usize] += val;
                j += j & -j;
            }
            i += i & -i;
        }
    }

    fn f(&self, row: i32, col: i32) -> i32 {
        let mut result = 0;
        let mut i = row + 1;
        while i > 0 {
            let mut j = col + 1;
            while j > 0 {
                result += self.bit[i as usize][j as usize];
                j -= j & -j;
            }
            i -= i & -i;
        }
        result
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.f(row2, col2) - self.f(row2, col1 - 1) - self.f(row1 - 1, col2) + self.f(row1 - 1, col1 - 1)
    }
}

/// 其他做法：每行保存一维的线段树或者树状数组，更新的时候，更新对应行。获取的时候，sum(每行对应列的和)
fn main() {
    let mut nm = NumMatrix::new(vec![vec![3, 0, 1, 4, 2], vec![5, 6, 3, 2, 1], vec![1, 2, 0, 1, 5], vec![4, 1, 0, 1, 7], vec![1, 0, 3, 0, 5]]);
    assert_eq!(nm.sum_region(2, 1, 4, 3), 8); // 返回 8 (即, 左侧红色矩形的和)
    nm.update(3, 2, 2);
    assert_eq!(nm.sum_region(2, 1, 4, 3), 10); // 返回 10 (即，右侧红色矩形的和)
}
