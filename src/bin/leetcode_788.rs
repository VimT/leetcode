//! 旋转数字

use std::sync::Once;

static mut PRE_SUM: Option<Vec<i32>> = None;
static ONCE: Once = Once::new();

pub fn rotated_digits(n: i32) -> i32 {
    ONCE.call_once(|| {
        let mut cur_sum = 0;
        let mut ps = vec![0; 10001];
        for i in 1..=10000 {
            let mut num = i;
            let mut ok = true;
            let mut c2569 = 0;
            while num > 0 {
                let wei = num % 10;
                if !matches!(wei, 0|1|2|5|6|8|9) {
                    ok = false;
                    break;
                }
                if matches!(wei, 2|5|6|9) {
                    c2569 += 1;
                }
                num /= 10;
            }
            if ok && c2569 > 0 { cur_sum += 1; }
            ps[i] = cur_sum;
        }
        unsafe { PRE_SUM = Some(ps); }
    });
    unsafe {
        let ps = PRE_SUM.as_ref().unwrap();
        ps[n as usize]
    }
}

/// 构造法dp
pub fn rotated_digits_dp(n: i32) -> i32 {
    let mut v = vec![];
    let mut num = n;
    while num > 0 {
        v.push(num % 10);
        num /= 10;
    }
    v.reverse();
    /// i 表示当前正在写第 i 位数字；
    /// equality_flag 表示已经写出的 j 位数字是否等于 N 的 j 位前缀；
    /// involution_flag 表示从最高位到比当前位高一位的这段前缀中是否含有 2569 中的任意一个数字。
    fn dfs(v: &Vec<i32>, cache: &mut Vec<Vec<Vec<i32>>>, i: usize, eq: bool, involution: bool) -> i32 {
        if i == v.len() { return involution as i32; }
        if cache[i][eq as usize][involution as usize] > 0 {
            return cache[i][eq as usize][involution as usize];
        }
        let mut result = 0;
        // 通过eq_flag，保证构造过程不超过n
        for d in 0..if eq { v[i] + 1 } else { 10 } {
            if matches!(d, 3|4|7) { continue; }
            result += dfs(v, cache, i + 1, eq && d == v[i], involution || matches!(d, 2|5|6|9));
        }
        cache[i][eq as usize][involution as usize] = result;
        result
    }
    dfs(&v, &mut vec![vec![vec![-1; 2]; 2]; v.len()], 0, true, false)
}

fn main() {
    assert_eq!(rotated_digits_dp(10), 4);
    assert_eq!(rotated_digits_dp(1), 0);
    assert_eq!(rotated_digits_dp(2), 1);
}
