//! 最少交换次数来组合所有的 1

pub fn min_swaps(data: Vec<i32>) -> i32 {
    let total: i32 = data.iter().sum();
    let mut cnt = data[..total as usize].iter().sum::<i32>();
    let mut result = cnt;
    for i in total as usize..data.len() {
        cnt += data[i];
        cnt -= data[i - total as usize];
        result = result.max(cnt);
        if result == total {
            return 0;
        }
    }
    total - result
}

fn main() {
    fn test(func: fn(data: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 0, 1, 0, 1]), 1);
        assert_eq!(func(vec![0, 0, 0, 1, 0]), 0);
        assert_eq!(func(vec![1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1]), 3);
    }
    test(min_swaps);
}
