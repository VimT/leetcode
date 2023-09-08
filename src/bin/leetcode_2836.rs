//! 在传球游戏中最大化函数值

/// 倍增 （时间优化：把倍增放第一维，因为cpu缓存的关系，时间48ms。 （放二维220ms））
pub fn get_max_function_value(receiver: Vec<i32>, k: i64) -> i64 {
    let len = receiver.len();
    let max: usize = 64 - k.leading_zeros() as usize;
    let mut skip = vec![vec![(0, 0); len]; max]; // skip[j][i] 表示从i开始，跳2^j步到达的位置和得分
    for i in 0..len {
        skip[0][i] = (receiver[i] as usize, receiver[i] as i64);
    }
    for i in 1..max {
        for j in 0..len {
            let (n1, s1) = skip[i - 1][j];
            let (n2, s2) = skip[i - 1][n1];
            skip[i][j] = (n2, s1 + s2);
        }
    }
    let mut result = 0;
    for i in 0..len {
        let mut this = i as i64;
        let mut cur = i;
        for j in (0..max).rev() {
            if k >> j & 1 == 1 {
                let (n, s) = skip[j][cur];
                this += s;
                cur = n;
            }
        }
        result = result.max(this);
    }
    result
}

fn main() {
    fn test(func: fn(receiver: Vec<i32>, k: i64) -> i64) {
        assert_eq!(func(vec![2, 0, 1], 4), 6);
        assert_eq!(func(vec![1, 1, 1, 2, 3], 3), 10);
    }
    test(get_max_function_value);
}
