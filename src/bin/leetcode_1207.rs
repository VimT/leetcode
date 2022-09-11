//! 独一无二的出现次数

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut cnt = [0; 2002];
    for num in arr {
        cnt[(num + 1000) as usize] += 1;
    }
    cnt.sort_unstable();
    for i in 1..2002 {
        if cnt[i] > 0 && cnt[i] == cnt[i - 1] { return false; }
    }
    true
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> bool) {
        assert_eq!(func(vec![1, 2, 2, 1, 1, 3]), true);
        assert_eq!(func(vec![1, 2]), false);
        assert_eq!(func(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]), true);
    }
    test(unique_occurrences);
}
