//! 添加边使所有节点度数都为偶数

use std::collections::HashSet;

pub fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    let mut set = HashSet::new();
    let mut degree = vec![0; n + 1];
    for edge in edges {
        degree[edge[0] as usize] += 1;
        degree[edge[1] as usize] += 1;
        set.insert((edge[0], edge[1]));
        set.insert((edge[1], edge[0]));
    }
    let mut odd = vec![];
    for d in 1..=n {
        if degree[d] & 1 == 1 {
            odd.push(d as i32);
        }
    }
    match odd.len() {
        0 => true,
        2 => {
            if !set.contains(&(odd[0], odd[1])) { return true; }
            for i in 1..=n as i32 {
                if odd[0] != i && odd[1] != i && !set.contains(&(i, odd[0])) && !set.contains(&(i, odd[1])) {
                    return true;
                }
            }
            false
        }
        4 => {
            let (a, b, c, d) = (odd[0], odd[1], odd[2], odd[3]);
            (!set.contains(&(a, b)) && !set.contains(&(c, d))) || (!set.contains(&(a, c)) && !set.contains(&(b, d))) || (!set.contains(&(a, d)) && !set.contains(&(b, c)))
        }
        _ => false
    }
}

/// 用 vec![HashSet::new(); n+1] 慢20ms
pub fn is_possible2(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    let mut set = vec![HashSet::new(); n + 1];
    let mut degree = vec![0; n + 1];
    for edge in edges {
        degree[edge[0] as usize] += 1;
        degree[edge[1] as usize] += 1;
        set[edge[0] as usize].insert(edge[1] as usize);
        set[edge[1] as usize].insert(edge[0] as usize);
    }
    let mut odd = vec![];
    for d in 1..=n {
        if degree[d] & 1 == 1 {
            odd.push(d);
        }
    }
    match odd.len() {
        0 => true,
        2 => {
            if !set[odd[0]].contains(&odd[1]) { return true; }
            for i in 1..=n {
                if odd[0] != i && odd[1] != i && !set[i].contains(&odd[0]) && !set[i].contains(&odd[1]) {
                    return true;
                }
            }
            false
        }
        4 => {
            let (a, b, c, d) = (odd[0], odd[1], odd[2], odd[3]);
            (!set[a].contains(&b) && !set[c].contains(&d)) || (!set[a].contains(&c) && !set[b].contains(&d)) || (!set[a].contains(&d) && !set[b].contains(&c))
        }
        _ => false
    }
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(21, vec![vec![2, 19], vec![16, 17], vec![8, 14], vec![2, 16], vec![12, 20], vec![12, 14], vec![16, 18], vec![15, 16], vec![10, 21], vec![3, 5], vec![13, 18], vec![17, 20], vec![14, 17], vec![9, 12], vec![5, 15], vec![5, 6], vec![3, 7], vec![2, 21], vec![10, 13], vec![8, 16], vec![7, 18], vec![4, 6], vec![9, 1], vec![13, 21], vec![18, 20], vec![7, 14], vec![4, 19], vec![5, 8], vec![3, 11], vec![11, 1], vec![7, 12], vec![4, 7], vec![3, 16], vec![13, 17], vec![17, 19], vec![9, 13], vec![7, 19], vec![10, 16], vec![4, 13], vec![4, 5], vec![2, 15], vec![12, 19], vec![11, 16], vec![2, 9], vec![11, 17], vec![17, 1], vec![16, 21], vec![4, 10], vec![10, 14], vec![14, 16], vec![4, 1], vec![13, 20], vec![5, 20], vec![4, 14], vec![4, 21], vec![10, 20], vec![2, 14], vec![8, 15], vec![4, 8], vec![6, 19], vec![15, 1], vec![19, 1], vec![8, 19], vec![15, 21], vec![3, 12], vec![11, 18], vec![9, 17], vec![18, 19], vec![7, 21], vec![3, 21], vec![16, 19], vec![11, 15], vec![5, 1], vec![8, 17], vec![3, 15], vec![8, 1], vec![10, 19], vec![3, 8], vec![6, 16], vec![2, 8], vec![5, 18], vec![11, 13], vec![11, 20], vec![14, 21], vec![6, 20], vec![4, 20], vec![12, 13], vec![5, 12], vec![10, 11], vec![9, 15], vec![3, 19], vec![9, 20], vec![14, 18], vec![21, 1], vec![13, 19], vec![8, 21], vec![2, 13], vec![3, 10], vec![9, 18], vec![19, 21], vec![6, 7], vec![3, 18], vec![2, 18], vec![6, 14], vec![3, 17], vec![5, 21], vec![14, 20], vec![8, 9], vec![16, 1], vec![3, 4], vec![13, 1], vec![5, 9], vec![4, 15], vec![17, 21], vec![20, 21], vec![2, 17], vec![13, 14], vec![11, 14], vec![9, 16], vec![10, 18], vec![6, 15], vec![6, 12], vec![3, 13], vec![5, 11], vec![6, 1], vec![12, 17], vec![8, 10], vec![5, 10], vec![8, 18], vec![4, 12], vec![10, 1], vec![6, 13], vec![4, 18], vec![7, 20], vec![7, 16], vec![2, 6], vec![12, 21], vec![4, 17], vec![15, 18], vec![13, 16], vec![15, 20], vec![7, 10], vec![6, 10], vec![2, 20], vec![7, 15], vec![18, 1], vec![12, 1], vec![3, 20], vec![7, 1], vec![14, 15], vec![4, 9], vec![11, 19], vec![7, 9], vec![5, 17], vec![18, 21], vec![6, 21], vec![8, 11], vec![6, 17], vec![3, 14], vec![7, 11], vec![5, 7], vec![7, 13], vec![6, 8], vec![6, 9], vec![10, 12], vec![5, 16], vec![2, 4], vec![17, 18], vec![9, 11], vec![12, 16], vec![3, 6], vec![12, 18], vec![3, 9], vec![11, 12], vec![14, 19], vec![10, 15], vec![5, 13], vec![8, 13], vec![15, 17], vec![2, 10], vec![11, 21], vec![20, 1], vec![6, 18], vec![2, 12], vec![19, 20], vec![6, 11], vec![8, 12], vec![2, 3], vec![12, 15], vec![2, 11], vec![9, 10], vec![7, 17], vec![9, 19], vec![13, 15], vec![7, 8], vec![4, 11], vec![2, 5], vec![5, 19], vec![16, 20], vec![15, 19], vec![9, 14], vec![14, 1], vec![10, 17], vec![9, 21], vec![2, 7], vec![8, 20], vec![5, 14], vec![4, 16]]), true);
        assert_eq!(func(11, vec![vec![5, 9], vec![8, 1], vec![2, 3], vec![7, 10], vec![3, 6], vec![6, 7], vec![7, 8], vec![5, 1], vec![5, 7], vec![10, 11], vec![3, 7], vec![6, 11], vec![8, 11], vec![3, 4], vec![8, 9], vec![9, 1], vec![2, 10], vec![9, 11], vec![5, 11], vec![2, 5], vec![8, 10], vec![2, 7], vec![4, 1], vec![3, 10], vec![6, 1], vec![4, 9], vec![4, 6], vec![4, 5], vec![2, 4], vec![2, 11], vec![5, 8], vec![6, 9], vec![4, 10], vec![3, 11], vec![4, 7], vec![3, 5], vec![7, 1], vec![2, 9], vec![6, 10], vec![10, 1], vec![5, 6], vec![3, 9], vec![2, 6], vec![7, 9], vec![4, 11], vec![4, 8], vec![6, 8], vec![3, 8], vec![9, 10], vec![5, 10], vec![2, 8], vec![7, 11]]), false);
        assert_eq!(func(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 2], vec![1, 4], vec![2, 5]]), true);
        assert_eq!(func(4, vec![vec![1, 2], vec![3, 4]]), true);
        assert_eq!(func(4, vec![vec![1, 2], vec![1, 3], vec![1, 4]]), false);
    }
    test(is_possible);
    test(is_possible2);
}
