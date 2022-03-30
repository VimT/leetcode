//! K 连续位的最小翻转次数

/// 差分数组
pub fn min_k_bit_flips(a: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut ans = 0;
    let len = a.len();
    let mut diff = vec![0; len + 1];
    let mut cnt = 0;
    for i in 0..len {
        cnt ^= diff[i];
        if a[i] == cnt {
            if i + k > len {
                return -1;
            }
            ans += 1;
            cnt ^= 1;
            diff[i + k] ^= 1;
        }
    }
    ans
}

fn main() {
    assert_eq!(min_k_bit_flips(vec![0, 1, 0], 1), 2);
    assert_eq!(min_k_bit_flips(vec![1, 1, 0], 2), -1);
    assert_eq!(min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3), 3);
}
