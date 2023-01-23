//! 循环码排列

pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
    let len = 1 << n as usize;
    let mut result = vec![0; len];
    result[0] = start;
    for i in 0..len as i32 {
        result[i as usize] = i ^ (i >> 1) ^ start;
    }
    result
}

/// 先89题生成格雷码，再旋转
pub fn circular_permutation2(n: i32, start: i32) -> Vec<i32> {
    let len = 1 << n as usize;
    let mut result = Vec::with_capacity(len);
    result.push(0);
    result.push(1);
    for i in 2..=n {
        for j in (0..result.len()).rev() {
            result.push(result[j] + (1 << (i - 1)));
        }
    }
    let s = (0..len).find(|&x| result[x] == start).unwrap();
    result[0..s].reverse();
    result[s..].reverse();
    result.reverse();
    result
}

fn main() {
    fn verify(func: fn(n: i32, start: i32) -> Vec<i32>, n: i32, start: i32) {
        let x = func(n, start);
        assert_eq!(x.len(), 1 << n as usize);
        assert_eq!(x[0], start);
        for i in 0..x.len() - 1 {
            assert_eq!((x[i] ^ x[i + 1]).count_ones(), 1);
        }
        assert_eq!((x[0] ^ x[x.len() - 1]).count_ones(), 1);
    }
    fn test(func: fn(n: i32, start: i32) -> Vec<i32>) {
        verify(func, 2, 3);
        verify(func, 3, 2);
    }
    test(circular_permutation);
    test(circular_permutation2);
}
