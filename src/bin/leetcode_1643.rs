//! 第 K 条最小指令

fn count_combinations(n: u64, r: u64) -> u64 {
    if r > n {
        0
    } else {
        (1..=r.min(n - r)).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

/// 组合数的递推式 c[n][k]=c[n−1][k−1]+c[n−1][k]
pub fn kth_smallest_path(destination: Vec<i32>, mut k: i32) -> String {
    let mut result = vec![];
    let mut h = destination[1];
    let mut v = destination[0];
    for _ in 0..h + v {
        if h > 0 {
            let o = count_combinations((h + v - 1) as u64, (h - 1) as u64) as i32;
            if k > o {
                result.push(b'V');
                v -= 1;
                k -= o;
            } else {
                result.push(b'H');
                h -= 1;
            }
        } else {
            result.push(b'V');
            v -= 1;
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(kth_smallest_path(vec![2, 3], 1), "HHHVV");
    assert_eq!(kth_smallest_path(vec![2, 3], 2), "HHVHV");
    assert_eq!(kth_smallest_path(vec![2, 3], 3), "HHVVH");
}
