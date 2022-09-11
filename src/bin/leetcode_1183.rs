//! 矩阵中 1 的最大数量

pub fn maximum_number_of_ones(width: i32, height: i32, side_length: i32, max_ones: i32) -> i32 {
    let mut res = vec![];
    for i in 0..side_length {
        for j in 0..side_length {
            // 正方形的这个点写1，扩展到矩阵后，可以写多少个1
            res.push(((width - i - 1) / side_length + 1) * ((height - j - 1) / side_length + 1));
        }
    }
    // 取最大max_ones个1，求和
    res.sort_unstable();
    res[res.len() - max_ones as usize..].iter().sum()
}

fn main() {
    fn test(func: fn(width: i32, height: i32, side_length: i32, max_ones: i32) -> i32) {
        assert_eq!(func(3, 3, 2, 1), 4);
        assert_eq!(func(3, 3, 2, 2), 6);
    }
    test(maximum_number_of_ones);
}
