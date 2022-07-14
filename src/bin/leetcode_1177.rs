//! 构建回文串检测

/// 位运算，前缀和
pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let s = s.as_bytes();
    let len = s.len();
    let mut pre_sum = vec![0i32; len + 1];
    for i in 0..len {
        pre_sum[i + 1] = pre_sum[i] ^ (1 << (s[i] - b'a'));
    }
    queries.into_iter().map(|x| {
        let (start, end, k) = (x[0], x[1], x[2]);
        let ones = (pre_sum[end as usize + 1] ^ pre_sum[start as usize]).count_ones();
        k as u32 >= ones / 2
    }).collect()
}

fn main() {
    fn test(func: fn(s: String, queries: Vec<Vec<i32>>) -> Vec<bool>) {
        assert_eq!(func(String::from("abcda"), vec![vec![3, 3, 0], vec![1, 2, 0], vec![0, 3, 1], vec![0, 3, 2], vec![0, 4, 1]]), vec![true, false, false, true, true]);
        assert_eq!(func(String::from("lyb"), vec![vec![0, 1, 0], vec![2, 2, 1]]), vec![false, true]);
    }
    test(can_make_pali_queries);
}
