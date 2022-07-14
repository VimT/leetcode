//! 道路的最大总重要性

pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let mut degree = vec![0; n as usize];
    for road in roads {
        degree[road[0] as usize] += 1;
        degree[road[1] as usize] += 1;
    }
    degree.sort_unstable();
    degree.into_iter().zip(1..=n).fold(0, |acc, (a, b)| acc + a as i64 * b as i64)
}

fn main() {
    fn test(func: fn(n: i32, roads: Vec<Vec<i32>>) -> i64) {
        assert_eq!(func(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 2], vec![1, 3], vec![2, 4]]), 43);
        assert_eq!(func(5, vec![vec![0, 3], vec![2, 4], vec![1, 3]]), 20);
    }
    test(maximum_importance);
}
