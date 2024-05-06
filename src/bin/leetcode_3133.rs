//! 数组最后一个元素的最小值

/// x里的1是固定位置，n的二进制填进x的空位
pub fn min_end(n: i32, x: i32) -> i64 {
    let mut bit = [0; 63];
    for i in 0..31 {
        if x >> i & 1 == 1 {
            bit[i] = 1;
        }
    }
    let mut j = 0;
    let n = n - 1;
    for i in 0..31 {
        while j < bit.len() && bit[j] == 1 {
            j += 1;
        }
        bit[j] = n >> i & 1;
        j += 1;
    }
    let mut result = 0;
    for i in 0..63 {
        if bit[i] == 1 {
            result |= 1 << i;
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, x: i32) -> i64) {
        assert_eq!(func(3, 4), 6);
        assert_eq!(func(2, 7), 15);
    }
    test(min_end);
}
