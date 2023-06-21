//! 找出数组游戏的赢家

pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut max = arr[0];
    let mut cnt = 0;
    for &num in &arr[1..] {
        if max > num {
            cnt += 1;
        } else {
            max = num;
            cnt = 1;
        }
        if cnt >= k { return max; }
    }
    max
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1,25,35,42,68,70], 1), 25);
        assert_eq!(func(vec![2, 1, 3, 5, 4, 6, 7], 2), 5);
        assert_eq!(func(vec![3, 2, 1], 10), 3);
        assert_eq!(func(vec![1, 9, 8, 2, 3, 7, 6, 4, 5], 7), 9);
        assert_eq!(func(vec![1, 11, 22, 33, 44, 55, 66, 77, 88, 99], 1000000000), 99);
    }
    test(get_winner);
}
