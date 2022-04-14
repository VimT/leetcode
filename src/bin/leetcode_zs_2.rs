//! 招商银行-02. 公园规划

/// 求最大度
pub fn num_flowers(roads: Vec<Vec<i32>>) -> i32 {
    let len = roads.len();
    let mut degree = vec![0; len + 1];
    for road in roads {
        degree[road[0] as usize] += 1;
        degree[road[1] as usize] += 1;
    }
    *degree.iter().max().unwrap() + 1
}

fn main() {
    assert_eq!(num_flowers(vec![vec![0, 1], vec![1, 3], vec![1, 2]]), 4);
    assert_eq!(num_flowers(vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 5], vec![3, 6], vec![5, 4]]), 3);
}
