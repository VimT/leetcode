//! k-avoiding 数组的最小总和

pub fn minimum_sum(n: i32, k: i32) -> i32 {
    let k = k as usize;
    let mut used = vec![false; 100];
    let mut i = 1;
    let mut result = 0;
    for _ in 0..n {
        while used[i] {
            i += 1;
        }
        used[i] = true;
        result += i;
        if k > i {
            used[k - i] = true;
        }
    }
    result as i32
}

/// 数学
pub fn minimum_sum2(n: i32, k: i32) -> i32 {
    let m = n.min(k / 2);
    (m * (m + 1) + (k * 2 + n - m - 1) * (n - m)) / 2
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> i32) {
        assert_eq!(func(5, 4), 18);
        assert_eq!(func(2, 6), 3);
    }
    test(minimum_sum);
    test(minimum_sum2);
}
