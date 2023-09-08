//! 所有排列中的最大和

pub fn max_sum_range_query(mut nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
    let len = nums.len();
    let mut counts = vec![0; len];
    for req in requests {
        let (start, end) = (req[0] as usize, req[1] as usize);
        counts[start] += 1;
        if end + 1 < len { counts[end + 1] -= 1; }
    }
    for i in 1..len {
        counts[i] += counts[i - 1];
    }

    counts.sort_unstable();
    nums.sort_unstable();
    let mut result = 0;
    const MOD: i64 = 1e9 as i64 + 7;
    for (num, count) in nums.into_iter().zip(counts).rev() {
        result = (result + num as i64 * count as i64) % MOD;
        if count == 0 { break; }
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4, 5], vec![vec![1, 3], vec![0, 1]]), 19);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6], vec![vec![0, 1]]), 11);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 10], vec![vec![0, 2], vec![1, 3], vec![1, 1]]), 47);
    }
    test(max_sum_range_query);
}
