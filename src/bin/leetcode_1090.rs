//! 受标签影响的最大值

pub fn largest_vals_from_labels(values: Vec<i32>, labels: Vec<i32>, num_wanted: i32, use_limit: i32) -> i32 {
    let mut v: Vec<(i32, usize)> = values.iter().enumerate().map(|(x, y)| (*y, x)).collect();
    v.sort_unstable();
    v.reverse();
    let mut m = std::collections::HashMap::new();
    let len = v.len();
    let mut result = 0;
    let mut cnt = 0;
    for i in 0..len {
        let (value, idx) = v[i];
        let label = labels[idx];
        if m.get(&label).unwrap_or(&0) < &use_limit {
            result += value;
            cnt += 1;
            *m.entry(label).or_insert(0) += 1;
        }
        if cnt == num_wanted {
            break;
        }
    }
    result
}

fn main() {
    assert_eq!(largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 1, 2, 2, 3], 3, 1), 9);
    assert_eq!(largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 3, 3, 3, 2], 3, 2), 12);
    assert_eq!(largest_vals_from_labels(vec![9, 8, 8, 7, 6], vec![0, 0, 0, 1, 1], 3, 1), 16);
    assert_eq!(largest_vals_from_labels(vec![9, 8, 8, 7, 6], vec![0, 0, 0, 1, 1], 3, 2), 24);
}
