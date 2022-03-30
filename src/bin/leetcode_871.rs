//! 最低加油次数

use std::collections::BinaryHeap;

pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
    let len = stations.len();
    if start_fuel >= target { return 0; }
    if (len == 0 && start_fuel < target) || (len > 0 && start_fuel < stations[0][0]) { return -1; }
    let mut heap = BinaryHeap::new();
    let mut cur = start_fuel;
    let mut result = 0;
    for i in 0..=len {
        let dis = if i == len { target - stations[len - 1][0] } else if i > 0 { stations[i][0] - stations[i - 1][0] } else { stations[0][0] };
        while dis > cur {
            if heap.is_empty() {
                return -1;
            }
            cur += heap.pop().unwrap();
            result += 1;
        }
        if i < len { heap.push(stations[i][1]); }
        cur -= dis;
    }
    result
}

fn main() {
    assert_eq!(min_refuel_stops(100, 10, vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]), 2);
    assert_eq!(min_refuel_stops(1, 1, vec![]), 0);
    assert_eq!(min_refuel_stops(100, 1, vec![vec![10, 100]]), -1);
}
