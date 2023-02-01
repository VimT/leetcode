//! 尽量减少恶意软件的传播

use leetcode::union_set::UnionSet;

pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
    let len = graph.len();
    let mut us = UnionSet::new(len);
    for i in 0..len {
        for j in i + 1..len {
            if graph[i][j] == 1 {
                us.union(i, j);
            }
        }
    }
    let mut init_root_cnt = vec![0; len];
    let mut result = initial[0];
    for &idx in &initial {
        init_root_cnt[us.find(idx as usize)] += 1;
        result = result.min(idx);
    }
    let mut max_remove_cnt = -1;
    for idx in initial {
        if init_root_cnt[us.find(idx as usize)] == 1 {
            let remove_cnt = us.size(idx as usize) as i32;
            if (remove_cnt > max_remove_cnt) || (remove_cnt == max_remove_cnt && idx < result) {
                result = idx;
                max_remove_cnt = remove_cnt;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(min_malware_spread(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]], vec![0, 1, 2]), 2);
    assert_eq!(min_malware_spread(vec![vec![1, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 1, 1], vec![0, 0, 1, 1]], vec![3, 1]), 3);
    assert_eq!(min_malware_spread(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]], vec![0, 1]), 0);
    assert_eq!(min_malware_spread(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]], vec![0, 2]), 0);
    assert_eq!(min_malware_spread(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], vec![1, 2]), 1);
}
