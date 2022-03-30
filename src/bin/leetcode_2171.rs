//! 拿出最少数目的魔法豆

pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
    beans.sort_unstable();
    let mut result = i64::MAX;
    let len = beans.len();
    let sum: i64 = beans.iter().map(|x| *x as i64).sum();
    for i in 0..len {
        result = result.min(sum - (len - i) as i64 * beans[i] as i64);
    }
    result
}

fn main() {
    assert_eq!(minimum_removal(vec![4, 1, 6, 5]), 4);
    assert_eq!(minimum_removal(vec![2, 10, 3, 2]), 7);
}
