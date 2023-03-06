//! 找出数组中的幸运数

pub fn find_lucky(arr: Vec<i32>) -> i32 {
    const LEN: usize = 501;
    let mut cnt = [0; LEN];
    for num in arr {
        cnt[num as usize] += 1;
    }
    for i in (1..LEN).rev() {
        if cnt[i] == i {
            return i as i32;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 2, 3, 4]), 2);
        assert_eq!(func(vec![1, 2, 2, 3, 3, 3]), 3);
        assert_eq!(func(vec![2, 2, 2, 3, 3]), -1);
    }
    test(find_lucky);
}
