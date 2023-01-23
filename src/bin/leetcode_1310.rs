//! 子数组异或查询

pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let len = arr.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] ^ arr[i];
    }
    queries.into_iter().map(|q| {
        presum[q[1] as usize + 1] ^ presum[q[0] as usize]
    }).collect()
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![1, 3, 4, 8], vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]), vec![2, 7, 14, 8]);
        assert_eq!(func(vec![4, 8, 2, 10], vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]]), vec![8, 0, 4, 4]);
    }
    test(xor_queries);
}
