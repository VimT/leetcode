//! 区间加法

pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
    let mut diff = vec![0; length as usize + 1];
    for update in updates {
        diff[update[0] as usize] += update[2];
        diff[update[1] as usize + 1] -= update[2];
    }
    let mut result = vec![0; length as usize];
    for i in 0..length as usize {
        result[i] = if i > 0 { result[i - 1] } else { 0 } + diff[i];
    }
    result
}

fn main() {
    fn test(func: fn(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(5, vec![vec![1, 3, 2], vec![2, 4, 3], vec![0, 2, -2]]), vec![-2, 0, 3, 5, 3]);
        assert_eq!(func(10, vec![vec![2, 4, 6], vec![5, 6, 8], vec![1, 9, -4]]), vec![0, -4, 2, 2, 2, 4, 4, -4, -4, -4]);
    }
    test(get_modified_array);
}
