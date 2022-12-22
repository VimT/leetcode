//! 查询树中环的长度

/// query(a,b) = 1 + (depth(a) - depth(LCA(ab))) + (depth(b) - depth(LCA(ab)))
pub fn cycle_length_queries(_: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    queries.into_iter().map(|query| {
        let (mut a, mut b) = (query[0], query[1]);
        let mut result = 1;
        while a != b {
            if a > b { a /= 2; } else { b /= 2; }
            result += 1;
        }
        result
    }).collect()
}

fn main() {
    fn test(func: fn(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(3, vec![vec![5, 3], vec![4, 7], vec![2, 3]]), vec![4, 5, 3]);
        assert_eq!(func(2, vec![vec![1, 2]]), vec![2]);
    }
    test(cycle_length_queries);
}
