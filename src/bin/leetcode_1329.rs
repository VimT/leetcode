//! 将矩阵按对角线排序

pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    for i in 0..m {
        let len = n.min(m - i);
        let mut arr = vec![0; len];
        for j in 0..len {
            arr[j] = mat[i + j][j];
        }
        arr.sort_unstable();
        for j in 0..len {
            mat[i + j][j] = arr[j];
        }
    }
    for j in 1..n {
        let len = m.min(n - j);
        let mut arr = vec![0; len];
        for i in 0..len {
            arr[i] = mat[i][i + j];
        }
        arr.sort_unstable();
        for i in 0..len {
            mat[i][i + j] = arr[i];
        }
    }
    mat
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]), vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]]);
        assert_eq!(func(vec![vec![11, 25, 66, 1, 69, 7], vec![23, 55, 17, 45, 15, 52], vec![75, 31, 36, 44, 58, 8], vec![22, 27, 33, 25, 68, 4], vec![84, 28, 14, 11, 5, 50]]), vec![vec![5, 17, 4, 1, 52, 7], vec![11, 11, 25, 45, 8, 69], vec![14, 23, 25, 44, 58, 15], vec![22, 27, 31, 36, 50, 66], vec![84, 28, 75, 33, 55, 68]]);
    }
    test(diagonal_sort);
}
