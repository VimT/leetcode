//! 边积分最高的节点

pub fn edge_score(edges: Vec<i32>) -> i32 {
    let len = edges.len();
    let mut sum = vec![0; len];
    for (s, &t) in edges.iter().enumerate() {
        sum[t as usize] += s;
    }
    let mut max_score = 0;
    let mut result = 0;
    for i in 0..len {
        if sum[i] > max_score {
            max_score = sum[i];
            result = i;
        }
    }
    result as i32
}

fn main() {
    assert_eq!(edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]), 7);
    assert_eq!(edge_score(vec![2, 0, 0, 2]), 0);
}
