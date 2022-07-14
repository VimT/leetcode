//! 校园自行车分配

use std::collections::BTreeMap;

pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> Vec<i32> {
    fn dis(a: &[i32], b: &[i32]) -> i32 {
        return (a[0] - b[0]).abs() + (a[1] - b[1]).abs();
    }
    let mut map: BTreeMap<i32, Vec<(usize, usize)>> = BTreeMap::new();
    for i in 0..workers.len() {
        for j in 0..bikes.len() {
            let distence = dis(&workers[i], &bikes[j]);
            map.entry(distence).or_default().push((i, j));
        }
    }
    let mut worker_assigned = vec![false; workers.len()];
    let mut bike_assigned = vec![false; bikes.len()];
    let mut result = vec![0; workers.len()];
    for (_, m) in map.into_iter() {
        for (w, b) in m {
            if worker_assigned[w] || bike_assigned[b] {
                continue;
            }
            worker_assigned[w] = true;
            bike_assigned[b] = true;
            result[w] = b as i32;
        }
    }
    result
}

fn main() {
    fn test(func: fn(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![0, 0], vec![2, 1]], vec![vec![1, 2], vec![3, 3]]), vec![1, 0]);
        assert_eq!(func(vec![vec![0, 0], vec![1, 1], vec![2, 0]], vec![vec![1, 0], vec![2, 2], vec![2, 1]]), vec![0, 2, 1]);
    }
    test(assign_bikes);
}
