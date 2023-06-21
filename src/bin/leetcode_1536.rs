//! 排布二进制网格的最少交换次数

pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut suffix_zero: Vec<usize> = grid.into_iter().map(|x| {
        let mut cnt = 0;
        for i in (0..n).rev() { if x[i] == 0 { cnt += 1; } else { break; } }
        cnt
    }).collect();

    let mut result = 0;
    for i in 0..n {
        if suffix_zero[i] < n - 1 - i {
            let mut swap = None;
            for j in i + 1..n {
                if suffix_zero[j] >= n - 1 - i {
                    swap = Some(j);
                    break;
                }
            }
            if let Some(j) = swap {
                for k in (i..j).rev() {
                    suffix_zero.swap(k, k + 1);
                }
                result += (j - i) as i32;
            } else { return -1; }
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]]), 3);
        assert_eq!(func(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0]]), -1);
        assert_eq!(func(vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 1]]), 0);
    }
    test(min_swaps);
}
