//! 黑格子的数目

use std::collections::HashMap;

pub fn count_black_blocks(m: i32, n: i32, coordinates: Vec<Vec<i32>>) -> Vec<i64> {
    static DIR: [(i32, i32); 4] = [(0, 0), (-1, -1), (-1, 0), (0, -1)];
    let mut cnt: HashMap<(i32, i32), i32> = HashMap::new();
    let mut result = vec![0; 5];
    for coo in coordinates {
        let (x, y) = (coo[0], coo[1]);
        for (dx, dy) in DIR {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && nx < m - 1 && ny >= 0 && ny < n - 1 {
                *cnt.entry((nx, ny)).or_default() += 1;
            }
        }
    }
    for (_, num) in cnt {
        result[num as usize] += 1;
    }
    result[0] = (m - 1) as i64 * (n - 1) as i64 - result.iter().sum::<i64>();
    result
}

fn main() {
    fn test(func: fn(m: i32, n: i32, coordinates: Vec<Vec<i32>>) -> Vec<i64>) {
        assert_eq!(func(3, 3, vec![vec![0, 0]]), vec![3, 1, 0, 0, 0]);
        assert_eq!(func(3, 3, vec![vec![0, 0], vec![1, 1], vec![0, 2]]), vec![0, 2, 2, 0, 0]);
    }
    test(count_black_blocks);
}
