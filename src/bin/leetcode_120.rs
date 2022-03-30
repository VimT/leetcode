//! 三角形最小路径和

pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut last_min = triangle.last().unwrap_or(&vec![]).clone();
    let len = triangle.len();
    for i in (0..len - 1).rev() {
        let mut tmp = vec![];
        for j in 0..=i {
            tmp.push(last_min[j].min(last_min[j + 1]) + triangle[i][j]);
        }
        last_min = tmp;
    }
    return last_min[0];
}

fn main() {
    assert_eq!(minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]), 11)
}