//! 移动石子直到连续 II

pub fn num_moves_stones_ii(mut stones: Vec<i32>) -> Vec<i32> {
    stones.sort_unstable();
    let len = stones.len();
    let max = stones[len - 1] - stones[0] + 1 - len as i32 - (stones[len - 1] - stones[len - 2] - 1).min(stones[1] - stones[0] - 1);
    let mut min = max;
    let mut j = 0;
    for i in 0..len {
        while j + 1 < len && stones[j + 1] - stones[i] + 1 <= len as i32 {
            j += 1;
        }
        let mut cost = len - (j - i + 1);
        if j - i + 1 == len - 1 && stones[j] - stones[i] + 1 == len as i32 - 1 {
            cost = 2;
        }
        min = min.min(cost as i32);
    }
    vec![min, max]
}

fn main() {
    assert_eq!(num_moves_stones_ii(vec![7, 4, 9]), vec![1, 2]);
    assert_eq!(num_moves_stones_ii(vec![6, 5, 4, 3, 10]), vec![2, 3]);
}
