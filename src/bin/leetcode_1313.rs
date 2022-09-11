//! 解压缩编码列表

pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for fv in nums.chunks(2) {
        for _ in 0..fv[0] {
            result.push(fv[1]);
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
        assert_eq!(func(vec![1, 1, 2, 3]), vec![1, 3, 3]);
    }
    test(decompress_rl_elist);
}
