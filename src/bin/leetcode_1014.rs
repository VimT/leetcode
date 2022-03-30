//! 最佳观光组合

pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let len = values.len();
    let mut result = 0;
    let mut cur_max = values[len - 1] - len as i32 + 1;
    for i in (0..len - 1).rev() {
        result = result.max(values[i] + i as i32 + cur_max);
        cur_max = cur_max.max(values[i] - i as i32);
    }
    result
}

fn main() {
    assert_eq!(max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]), 11);
    assert_eq!(max_score_sightseeing_pair(vec![1, 2]), 2);
}
