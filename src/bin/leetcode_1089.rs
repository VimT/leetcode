//! 复写零

pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let len = arr.len();
    let zero_cnt = arr.iter().filter(|x| **x == 0).count();
    arr.resize(len + zero_cnt, 0);
    let mut j = arr.len();
    for i in (0..len).rev() {
        if arr[i] == 0 {
            arr[j - 1] = 0;
            arr[j - 2] = 0;
            j -= 2;
        } else {
            arr[j - 1] = arr[i];
            j -= 1;
        }
    }
    arr.truncate(len)
}

fn main() {
    fn test(func: fn(arr: &mut Vec<i32>)) {
        let help = |mut arr: Vec<i32>| -> Vec<i32> {
            func(&mut arr);
            arr
        };
        assert_eq!(help(vec![1, 0, 2, 3, 0, 4, 5, 0]), [1, 0, 0, 2, 3, 0, 0, 4]);
        assert_eq!(help(vec![1, 2, 3]), [1, 2, 3]);
    }
    test(duplicate_zeros);
}
