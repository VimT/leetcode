//! 相对名次

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut s: Vec<(i32, usize)> = score.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
    s.sort_unstable();
    s.reverse();
    let mut result = vec![String::new(); s.len()];
    for (rank, (_, idx)) in s.into_iter().enumerate() {
        let rank = rank + 1;
        result[idx] = match rank {
            1 => String::from("Gold Medal"),
            2 => String::from("Silver Medal"),
            3 => String::from("Bronze Medal"),
            _ => rank.to_string(),
        }
    }
    result
}

fn main() {
    fn test(func: fn(score: Vec<i32>) -> Vec<String>) {
        assert_eq!(func(vec![5, 4, 3, 2, 1]), vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]);
        assert_eq!(func(vec![10, 3, 8, 9, 4]), vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]);
    }
    test(find_relative_ranks);
}
