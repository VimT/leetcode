//! 统计将重叠区间合并成组的方案数

/// 区间合并
pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
    ranges.sort_unstable();
    const MOD: i32 = 1e9 as i32 + 7;
    let mut result = 1;
    let mut max = -1;
    for range in ranges {
        if range[0] > max {
            result = (result * 2) % MOD;
        }
        max = max.max(range[1]);
    }
    result
}

fn main() {
    fn test(func: fn(ranges: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 1]]), 2);
        assert_eq!(func(vec![vec![5, 11], vec![20, 22], vec![1, 3], vec![21, 22], vec![11, 11]]), 8);
        assert_eq!(func(vec![vec![6, 10], vec![5, 15]]), 2);
        assert_eq!(func(vec![vec![1, 3], vec![10, 20], vec![2, 5], vec![4, 8]]), 4);
    }
    test(count_ways);
}
