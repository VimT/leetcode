//! 最小绝对差

pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    arr.sort_unstable();
    let mut result = vec![];
    let mut min_diff = i32::MAX;
    for one in arr.windows(2) {
        let diff = one[1] - one[0];
        if diff < min_diff {
            min_diff = diff;
            result.clear();
        }
        if diff == min_diff {
            result.push(one.to_vec());
        }
    }
    result
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![4, 2, 1, 3]), vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
        assert_eq!(func(vec![1, 3, 6, 10, 15]), vec![vec![1, 3]]);
        assert_eq!(func(vec![3, 8, -10, 23, 19, -4, -14, 27]), vec![vec![-14, -10], vec![19, 23], vec![23, 27]]);
    }
    test(minimum_abs_difference);
}
