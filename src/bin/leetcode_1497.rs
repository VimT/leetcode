//! 检查数组对是否可以被 k 整除


pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    let mut cnt = vec![0; k as usize];
    for mut num in arr {
        num = (num % k + k) % k;
        cnt[num as usize] += 1;
    }
    for i in 1..=k / 2 {
        if cnt[i as usize] != cnt[(k - i) as usize] {
            return false;
        }
    }
    cnt[0] & 1 == 0
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, k: i32) -> bool) {
        assert_eq!(func(vec![0, 2, 2, 2], 4), false);
        assert_eq!(func(vec![-1, -1, -1, -1, 2, 2, -2, -2], 3), false);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5), true);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6], 7), true);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6], 10), false);
    }
    test(can_arrange);
}
