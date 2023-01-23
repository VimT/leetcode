//! 重构 2 行二进制矩阵

pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let len = colsum.len();
    let mut result = vec![vec![0; len]; 2];
    for i in 0..len {
        match colsum[i] {
            0 => (),
            1 => {
                if upper >= lower {
                    upper -= 1;
                    result[0][i] = 1;
                } else {
                    lower -= 1;
                    result[1][i] = 1;
                }
            }
            2 => {
                upper -= 1;
                lower -= 1;
                result[0][i] = 1;
                result[1][i] = 1;
            }
            _ => unreachable!()
        }
        if upper < 0 || lower < 0 {
            return vec![];
        }
    }
    if upper != 0 || lower != 0 { return vec![]; }
    result
}

fn main() {
    fn test(func: fn(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>>) {
        assert_eq!(func(2, 1, vec![1, 1, 1]), vec![vec![1, 1, 0], vec![0, 0, 1]]);
        assert_eq!(func(2, 3, vec![2, 2, 1, 1]), Vec::<Vec<i32>>::new());
        assert_eq!(func(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1]), [[1, 1, 1, 0, 0, 0, 1, 1, 0, 0], [1, 0, 1, 0, 1, 0, 0, 1, 0, 1]]);
    }
    test(reconstruct_matrix);
}
