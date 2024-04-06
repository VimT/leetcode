//! 得到更多分数的最少关卡数目

pub fn minimum_levels(mut possible: Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in &mut possible {
        if *num == 0 { *num = -1; }
        sum += *num;
    }
    let len = possible.len();
    let mut cur = 0;
    for i in 0..len - 1 {
        cur += possible[i];
        if cur > sum - cur { return i as i32 + 1; }
    }
    -1
}

fn main() {
    fn test(func: fn(possible: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 0, 1, 0]), 1);
        assert_eq!(func(vec![1, 1, 1, 1, 1]), 3);
        assert_eq!(func(vec![0, 0]), -1);
    }
    test(minimum_levels);
}
