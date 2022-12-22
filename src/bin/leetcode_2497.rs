//! 图中最大星和

pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
    let len = vals.len();
    let mut star = vec![vec![]; len];
    for edge in edges {
        let v1 = vals[edge[1] as usize];
        let v2 = vals[edge[0] as usize];
        if v1 > 0 { star[edge[0] as usize].push(v1) };
        if v2 > 0 { star[edge[1] as usize].push(v2) };
    }
    let mut result = i32::MIN;
    for i in 0..len {
        let mut cur = vals[i];
        let v = &mut star[i];
        v.sort_unstable();
        for &val in v.iter().rev().take(k as usize) {
            cur += val;
        }
        result = result.max(cur);
    }
    result
}

fn main() {
    assert_eq!(max_star_sum(vec![1, -8, 0], vec![vec![1, 0], vec![2, 1]], 2), 1);
    assert_eq!(max_star_sum(vec![1, 2, 3, 4, 10, -10, -20], vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5], vec![3, 6]], 2), 16);
    assert_eq!(max_star_sum(vec![-5], vec![], 0), -5);
}
