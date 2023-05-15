//! 找出叠涂元素

pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut row = vec![0; m];
    let mut col = vec![0; n];
    let len = m * n;
    let mut map = vec![(0, 0); len + 1];
    for i in 0..m {
        for j in 0..n {
            map[mat[i][j] as usize] = (i, j);
        }
    }
    for (i, num) in arr.into_iter().enumerate() {
        let (x, y) = map[num as usize];
        row[x] += 1;
        col[y] += 1;
        if row[x] == n || col[y] == m {
            return i as i32;
        }
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]]), 2);
        assert_eq!(func(vec![2, 8, 7, 4, 1, 3, 5, 6, 9], vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]]), 3);
    }
    test(first_complete_index);
}
