//! 统计好三元组

pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let len = arr.len();
    let mut result = 0;
    for i in 0..len {
        for j in i + 1..len {
            for k in j + 1..len {
                if (arr[i] - arr[j]).abs() <= a && (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                    result += 1;
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32) {
        assert_eq!(func(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
        assert_eq!(func(vec![1, 1, 2, 2, 3], 0, 0, 1), 0);
    }
    test(count_good_triplets);
}
