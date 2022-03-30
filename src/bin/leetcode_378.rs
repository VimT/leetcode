//! 有序矩阵中第 K 小的元素

/// 有序矩阵中第K小的元素
/// 根据值 找数量。
/// 另外思路：根据索引，找数量
pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    /// 找升序矩阵，小于某个数的数量， 某个数左边上边的所有 + 右边上面的部分。
    /// 通过从左下角开始遍历，
    fn find_not_bigger_than(matrix: &Vec<Vec<i32>>, find: i32) -> usize {
        let mut x = matrix.len() - 1;
        let mut y = 0;
        let mut count = 0;

        while y < matrix[0].len() {
            if matrix[x][y] <= find {
                y += 1;
                // 第y列有x+1个元素 <= find
                count += x + 1;
            } else {
                if x == 0 { break; }
                x -= 1;
            }
        }
        count
    }

    let mut left = matrix[0][0];
    let mut right = *matrix.last().unwrap().last().unwrap();

    while left < right {
        let mid = (left + right) / 2;
        let find = find_not_bigger_than(&matrix, mid);
        if (find as i32) < k {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return left;
}


fn main() {
    assert_eq!(kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8), 13);
    assert_eq!(kth_smallest(vec![vec![-5]], 1), -5);
}
