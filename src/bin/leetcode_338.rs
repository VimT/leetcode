//! 比特位计数

pub fn count_bits(num: i32) -> Vec<i32> {
    let mut ans = vec![0; num as usize + 1];
    for i in 1..=num as usize {
        ans[i] = ans[i & (i - 1)] + 1;
    }
    ans
}

pub fn count_bits_1(num: i32) -> Vec<i32> {
    let mut ans = vec![0; num as usize + 1];
    for i in 1..=num as usize {
        ans[i] = ans[i >> 1] + (1 & i as i32);
        // println!("{}:{}, {}", i, ans[i], ans[i >> 1]);
    }
    ans
}


fn main() {
    fn test(func: fn(n: i32) -> Vec<i32>) {
        assert_eq!(func(2), vec![0, 1, 1]);
        assert_eq!(func(5), vec![0, 1, 1, 2, 1, 2]);
    }
    test(count_bits);
    test(count_bits_1);
}
