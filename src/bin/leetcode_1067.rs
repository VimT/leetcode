//! 范围内的数字计数


/// 数学归纳法、找规律、纯数学
pub fn digits_count(d: i32, low: i32, high: i32) -> i32 {
    /// 找数字d在1-n出现的次数
    fn count(n: i32, d: i32) -> i32 {
        let mut cnt = 0;
        let mut i = 1;
        let mut k = n;
        while k != 0 {
            // 高位的数字
            let mut high = k / 10;
            if d == 0 {
                if high != 0 {
                    high -= 1;
                } else {
                    break;
                }
            }
            cnt += high * i;
            // 当前位的数字
            let cur = k % 10;
            if cur > d {
                cnt += i;
            } else if cur == d {
                // 低位的数字
                cnt += n - k * i + 1;
            }
            i *= 10;
            k = n / i;
        }
        cnt
    }
    count(high, d) - count(low - 1, d)
}

fn main() {
    fn test(func: fn(d: i32, low: i32, high: i32) -> i32) {
        assert_eq!(func(1, 1, 13), 6);
        assert_eq!(func(3, 100, 250), 35);
    }
    test(digits_count);
}
