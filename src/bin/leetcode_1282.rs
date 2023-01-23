//! 用户分组

pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut help = vec![vec![]; 501];
    for (i, size) in group_sizes.into_iter().enumerate() {
        help[size as usize].push(i as i32);
        if help[size as usize].len() == size as usize {
            result.push(help[size as usize].clone());
            help[size as usize].clear();
        }
    }
    result
}

fn main() {
    fn test(func: fn(group_sizes: Vec<i32>) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![3, 3, 3, 3, 3, 1, 3]), vec![vec![0, 1, 2], vec![5], vec![3, 4, 6]]);
        assert_eq!(func(vec![2, 1, 3, 3, 3, 2]), vec![vec![1], vec![2, 3, 4], vec![0, 5]]);
    }
    test(group_the_people);
}
