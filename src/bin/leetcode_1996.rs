//! 游戏中弱角色的数量

pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
    properties.sort_unstable_by_key(|x| (x[0], -x[1]));
    let mut s = vec![];
    let mut result = 0;
    for pp in properties {
        while !s.is_empty() && *s.last().unwrap() < pp[1] {
            result += 1;
            s.pop().unwrap();
        }
        s.push(pp[1]);
    }
    result
}

fn main() {
    assert_eq!(number_of_weak_characters(vec![vec![1, 1], vec![2, 1], vec![2, 2], vec![1, 2]]), 1);
    assert_eq!(number_of_weak_characters(vec![vec![5, 5], vec![5, 4], vec![5, 3]]), 0);
    assert_eq!(number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]), 0);
    assert_eq!(number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]), 1);
    assert_eq!(number_of_weak_characters(vec![vec![2, 2], vec![2, 3]]), 0);
    assert_eq!(number_of_weak_characters(vec![vec![2, 2], vec![2, 2]]), 0);
    assert_eq!(number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]), 1);
}
