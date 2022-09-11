//! 数组序号转换

pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut na: Vec<(usize, i32)> = arr.into_iter().enumerate().collect();
    na.sort_unstable_by_key(|x| x.1);
    let mut result = vec![0; na.len()];
    let mut rank = 0;
    let mut last = i32::MAX;
    for (i, num) in na {
        if num != last {
            rank += 1;
            last = num;
        }
        result[i] = rank;
    }
    result
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![40, 10, 20, 30]), vec![4, 1, 2, 3]);
        assert_eq!(func(vec![100, 100, 100]), vec![1, 1, 1]);
        assert_eq!(func(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]), vec![5, 3, 4, 2, 8, 6, 7, 1, 3]);
    }
    test(array_rank_transform);
}
