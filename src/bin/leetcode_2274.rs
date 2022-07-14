//! 不含特殊楼层的最大连续楼层数

pub fn max_consecutive(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
    special.push(bottom - 1);
    special.push(top + 1);
    special.sort_unstable();
    special.windows(2).map(|x| x[1] - x[0] - 1).max().unwrap()
}

fn main() {
    fn test(func: fn(bottom: i32, top: i32, special: Vec<i32>) -> i32) {
        assert_eq!(func(2, 9, vec![4, 6]), 3);
        assert_eq!(func(6, 8, vec![7, 6, 8]), 0);
    }
    test(max_consecutive);
}
