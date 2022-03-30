//! 回旋镖的数量

use std::collections::HashMap;

pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
    let len = points.len();
    let mut m: HashMap<i64, i32> = HashMap::new();
    let mut result = 0;
    for i in 0..len {
        m.clear();
        for j in 0..len {
            let dx = (points[i][0] - points[j][0]) as i64;
            let dy = (points[i][1] - points[j][1]) as i64;
            let dis = dx * dx + dy * dy;
            let count = m.entry(dis).or_insert(0);
            result += 2 * (*count);
            *count += 1;
        }
    }
    result as i32
}

fn main() {
    assert_eq!(number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]), 2);
    assert_eq!(number_of_boomerangs(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 2);
    assert_eq!(number_of_boomerangs(vec![vec![1, 1]]), 0);
}
