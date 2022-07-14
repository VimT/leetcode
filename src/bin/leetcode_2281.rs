//! 巫师的总力量和

/// 单调栈找左边和右边第一个比 自己 小的 区间
/// 这个区间的所有子数组的和（ for li in l..i: for ri in i..r: sum += presum[ri] - presum[li] )
/// 写成公式后发现可以用 前缀和前缀和 数组加速
/// 最后公式为：(i−L+1)⋅(ss[R+2]−ss[i+1])−(R−i+1)⋅(ss[i+1]−ss[L])
pub fn total_strength(strength: Vec<i32>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let len = strength.len();
    let mut left = vec![-1; len];
    let mut right = vec![len as i64; len];
    let mut st = vec![];
    for i in 0..len {
        while !st.is_empty() && strength[*st.last().unwrap()] >= strength[i] {
            right[st.pop().unwrap()] = i as i64;
        }
        if !st.is_empty() {
            left[i] = *st.last().unwrap() as i64;
        }
        st.push(i);
    }
    let mut ss = vec![0; len + 2];
    let mut s = 0;
    for i in 1..=len {
        s += strength[i - 1] as i64;
        ss[i + 1] = (ss[i] + s) % MOD;
    }
    let mut result = 0;
    for i in 0..len {
        let l = left[i] + 1;
        let r = right[i] - 1;
        let tot = ((i as i64 - l + 1) * (ss[r as usize + 2] - ss[i + 1]) - (r + 1 - i as i64) * (ss[i + 1] - ss[l as usize])) % MOD;
        result = (result + strength[i] as i64 * tot) % MOD;
    }
    ((result + MOD) % MOD) as i32
}

fn main() {
    fn test(func: fn(strength: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 3, 1, 2]), 44);
        assert_eq!(func(vec![5, 4, 6]), 213);
    }
    test(total_strength);
}
