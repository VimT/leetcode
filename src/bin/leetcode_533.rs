//! 孤独像素 II

use std::collections::HashMap;

pub fn find_black_pixel(picture: Vec<Vec<char>>, target: i32) -> i32 {
    let m = picture.len();
    let n = picture[0].len();
    let mut rows = vec![vec![]; m];
    let mut cols = vec![vec![]; n];
    let mut row_map = HashMap::new();
    for i in 0..m {
        row_map.entry(&picture[i]).or_insert(vec![]).push(i);
        for j in 0..n {
            if picture[i][j] == 'B' {
                rows[i].push(j);
                cols[j].push(i);
            }
        }
    }
    let mut ok_cols = vec![false; n];
    for j in 0..n {
        if cols[j].len() == target as usize {
            if let Some(same_rows) = row_map.get(&picture[cols[j][0]]) {
                // 判断每个 black 所在的行是否完全相同，更简单的做法：并查集。
                let mut ok = true;
                let mut k = 0;
                for &i in &cols[j] {
                    while k < same_rows.len() && same_rows[k] < i {
                        k += 1;
                    }
                    if k == same_rows.len() || same_rows[k] != i {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    ok_cols[j] = true;
                }
            }
        }
    }
    let mut result = 0;
    for i in 0..m {
        if rows[i].len() == target as usize {
            for &j in &rows[i] {
                if ok_cols[j] {
                    result += 1;
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(picture: Vec<Vec<char>>, target: i32) -> i32) {
        assert_eq!(func(vec![vec!['W', 'B', 'W', 'B', 'B', 'W'], vec!['W', 'B', 'W', 'B', 'B', 'W'], vec!['W', 'B', 'W', 'B', 'B', 'W'], vec!['W', 'W', 'B', 'W', 'B', 'W']], 3), 6);
        assert_eq!(func(vec![vec!['W', 'W', 'B'], vec!['W', 'W', 'B'], vec!['W', 'W', 'B']], 1), 0);
    }
    test(find_black_pixel);
}
