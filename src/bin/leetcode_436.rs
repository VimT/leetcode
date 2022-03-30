//! 寻找右区间

pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let len = intervals.len();
    let mut idxs: Vec<usize> = (0..len).collect();
    idxs.sort_unstable_by_key(|&x| intervals[x][0]);
    let mut result = vec![0; len];
    for i in 0..len {
        let cc = idxs.binary_search_by_key(&intervals[i][1], |&x| intervals[x][0]).unwrap_or_else(|x| x);
        result[i] = if cc < len { idxs[cc] as i32 } else { -1 };
    }
    result
}

fn main() {
    assert_eq!(find_right_interval(vec![vec![1, 2]]), vec![-1]);
    assert_eq!(find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]), vec![-1, 0, 1]);
    assert_eq!(find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]), vec![-1, 2, -1]);
}
