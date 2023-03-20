//! 执行操作后的最大 MEX

pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
    let mut cnt = vec![0; value as usize + 1];
    for num in nums {
        cnt[((num % value + value) % value) as usize] += 1;
    }
    let value = value as usize;
    let mut mex = 0;
    while cnt[mex % value] > 0 {
        cnt[mex % value] -= 1;
        mex += 1;
    }
    mex as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, value: i32) -> i32) {
        assert_eq!(func(vec![1, -10, 7, 13, 6, 8], 7), 2);
        assert_eq!(func(vec![1, -10, 7, 13, 6, 8], 5), 4);
    }
    test(find_smallest_integer);
}
