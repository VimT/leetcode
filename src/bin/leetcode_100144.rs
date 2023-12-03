//! 找出峰值

pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    let len = mountain.len();
    let mut result = vec![];
    for i in 1..len - 1 {
        if mountain[i] > mountain[i - 1] && mountain[i] > mountain[i + 1] {
            result.push(i as i32);
        }
    }
    result
}

fn main() {
    fn test(func: fn(mountain: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![2, 4, 4]), vec![]);
        assert_eq!(func(vec![1, 4, 3, 8, 5]), vec![1, 3]);
    }
    test(find_peaks);
}
