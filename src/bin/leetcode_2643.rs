//! 一最多的行

pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut max_line = 0;
    let mut max = 0;
    for (i, line) in mat.into_iter().enumerate() {
        let one: i32 = line.iter().sum();
        if one > max {
            max_line = i;
            max = one;
        }
    }
    vec![max_line as i32, max]
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![0, 1], vec![1, 0]]), vec![0, 1]);
        assert_eq!(func(vec![vec![0, 0, 0], vec![0, 1, 1]]), vec![1, 2]);
        assert_eq!(func(vec![vec![0, 0], vec![1, 1], vec![0, 0]]), vec![1, 2]);
    }
    test(row_and_maximum_ones);
}
