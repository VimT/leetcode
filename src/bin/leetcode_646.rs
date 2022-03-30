//! 最长数对链

pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
    pairs.sort_unstable_by_key(|x| (x[1], x[0]));
    let mut cur = i32::MIN;
    let mut result = 0;
    for pair in pairs {
        if cur < pair[0] {
            cur = pair[1];
            result += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(find_longest_chain(vec![vec![9, 10], vec![2, 9], vec![-1, 9], vec![0, 6], vec![-4, 8], vec![7, 8], vec![3, 10], vec![0, 6], vec![0, 7], vec![-1, 5]]), 3);
    assert_eq!(find_longest_chain(vec![vec![7, 9], vec![4, 5], vec![7, 9], vec![-7, -1], vec![0, 10], vec![3, 10], vec![3, 6], vec![2, 3]]), 4);
    assert_eq!(find_longest_chain(vec![vec![9, 10], vec![9, 10], vec![4, 5], vec![-9, -3], vec![-9, 1], vec![0, 3], vec![6, 10], vec![-5, -4], vec![-7, -6]]), 5);
    assert_eq!(find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]), 2);
    assert_eq!(find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]), 3);
}
