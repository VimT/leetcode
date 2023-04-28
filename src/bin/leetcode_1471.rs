//! 数组中的 k 个最强值

pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
    arr.sort_unstable();
    let len = arr.len();
    let mid = arr[(len - 1) / 2];
    let mut result = vec![0; k as usize];
    let mut i = 0;
    let mut j = len;
    for x in 0..k as usize {
        if mid - arr[i] > arr[j - 1] - mid {
            result[x] = arr[i];
            i += 1;
        } else {
            result[x] = arr[j - 1];
            j -= 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, k: i32) -> Vec<i32>) {
        assert_eq!(func(vec![-7, 22, 17, 3], 2), vec![22, 17]);
        assert_eq!(func(vec![1, 2, 3, 4, 5], 2), vec![5, 1]);
        assert_eq!(func(vec![1, 1, 3, 5, 5], 2), vec![5, 5]);
        assert_eq!(func(vec![6, 7, 11, 7, 6, 8], 5), vec![11, 8, 6, 6, 7]);
    }
    test(get_strongest);
}
