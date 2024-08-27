//! 放三个车的价值之和最大 I

use std::collections::HashSet;

pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
    let m = board.len();
    let n = board[0].len();
    fn top_three(vec: Vec<(i32, usize, usize)>) -> Vec<(i32, usize, usize)> {
        let mut result = vec![(i32::MIN, 0, 0); 3];
        for (v, r, c) in vec {
            if v > result[0].0 {
                result[2] = result[1];
                result[1] = result[0];
                result[0] = (v, r, c);
            } else if v > result[1].0 {
                result[2] = result[1];
                result[1] = (v, r, c);
            } else if v > result[2].0 {
                result[2] = (v, r, c);
            }
        }
        result
    }

    // 每一行的 top3
    let rows: Vec<Vec<(i32, usize, usize)>> = (0..m)
        .map(|i| top_three((0..n).map(|j| (board[i][j], i, j)).collect()))
        .collect();

    // 每一列的 top3
    let cols: Vec<Vec<(i32, usize, usize)>> = (0..n)
        .map(|j| top_three((0..m).map(|i| (board[i][j], i, j)).collect()))
        .collect();

    // 行和列取交集，保留 top9
    let mut s: Vec<(i32, usize, usize)> = rows.into_iter().flatten().collect::<HashSet<(i32, usize, usize)>>()
        .intersection(&cols.into_iter().flatten().collect::<HashSet<(i32, usize, usize)>>())
        .copied().collect();
    s.sort_unstable_by_key(|x| -x.0);
    s.truncate(9);

    let mut result = i64::MIN;
    for i in 0..s.len() {
        for j in i + 1..s.len() {
            for k in j + 1..s.len() {
                let (v1, r1, c1) = s[i];
                let (v2, r2, c2) = s[j];
                let (v3, r3, c3) = s[k];
                if r1 != r2 && r1 != r3 && r2 != r3 && c1 != c2 && c1 != c3 && c2 != c3 {
                    result = result.max(v1 as i64 + v2 as i64 + v3 as i64);
                }
            }
        }
    }

    result
}

fn main() {
    fn test(func: fn(board: Vec<Vec<i32>>) -> i64) {
        assert_eq!(func(vec![
            vec![110572640, 264484041, 191937624, 107536912, 204414029],
            vec![6966832, 292174508, 166104330, 172054792, 145749354],
            vec![100851493, 357264852, 382618249, 275156564, 312233661],
            vec![231460367, 290246601, 132848045, 148430740, 164965796],
            vec![52372190, 316570467, 288950457, 176606578, 181671468]
        ]), 930649083);
        assert_eq!(func(vec![vec![5, 29, 12], vec![23, -37, 99], vec![12, 31, 57]]), 140);
        assert_eq!(func(vec![vec![-3, 1, 1, 1], vec![-3, 1, -3, 1], vec![-3, 2, 1, 1]]), 4);
        assert_eq!(func(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), 15);
        assert_eq!(func(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]), 3);
    }
    test(maximum_value_sum);
}
