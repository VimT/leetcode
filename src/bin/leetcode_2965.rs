//! 找出缺失和重复的数字

pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let len = grid.len();
    let mut cnt = vec![0; len * len];
    for i in 0..len {
        for j in 0..len {
            cnt[grid[i][j] as usize - 1] += 1;
        }
    }
    let mut result = vec![0; 2];
    for i in 0..len * len {
        if cnt[i] == 0 {
            result[1] = i as i32 + 1;
        } else if cnt[i] == 2 {
            result[0] = i as i32 + 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 3], vec![2, 2]]), vec![2, 4]);
        assert_eq!(func(vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]]), vec![9, 5]);
    }
    test(find_missing_and_repeated_values);
}
