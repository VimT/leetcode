//! 将珠子放入背包中

/// 往中间插k-1个空，看插完的最大和最小
pub fn put_marbles(mut weights: Vec<i32>, k: i32) -> i64 {
    let len = weights.len();
    for i in 0..len - 1 {
        weights[i] += weights[i + 1];
    }
    weights.pop();
    weights.sort_unstable();
    weights[len - k as usize..].iter().map(|&x| x as i64).sum::<i64>() - weights[..k as usize - 1].iter().map(|&x| x as i64).sum::<i64>()
}

fn main() {
    fn test(func: fn(weights: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![1, 3, 5, 1], 2), 4);
        assert_eq!(func(vec![1, 3], 2), 0);
    }
    test(put_marbles);
}
