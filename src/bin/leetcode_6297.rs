//! 根据第 K 场考试的分数排序

pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    score.sort_unstable_by_key(|x| -x[k as usize]);
    score
}

fn main() {
    fn test(func: fn(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]], 2), vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]]);
        assert_eq!(func(vec![vec![3, 4], vec![5, 6]], 0), vec![vec![5, 6], vec![3, 4]]);
    }
    test(sort_the_students);
}
