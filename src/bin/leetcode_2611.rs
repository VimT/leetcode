//! 老鼠和奶酪

pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
    let len = reward2.len();
    let mut diff: Vec<(i32, usize)> = (0..len).map(|x| reward2[x] - reward1[x]).zip(0..).collect();
    diff.sort_unstable();
    let mut result = 0;
    for i in 0..k as usize {
        result += reward1[diff[i].1];
    }
    for i in k as usize..len {
        result += reward2[diff[i].1];
    }
    result
}

fn main() {
    fn test(func: fn(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 1, 3, 4], vec![4, 4, 1, 1], 2), 15);
        assert_eq!(func(vec![1, 1], vec![1, 1], 2), 2);
    }
    test(mice_and_cheese);
}
