//! 乘法表中第k小的数

pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
    let mut left = 1;
    let mut right = m * n;
    while left < right {
        let mid = (left + right) / 2;
        let mut cnt = 0;
        for i in 1..=m {
            cnt += n.min(mid / i);
        }
        if cnt >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    assert_eq!(find_kth_number(3, 3, 5), 3);
    assert_eq!(find_kth_number(2, 3, 6), 6);
}
