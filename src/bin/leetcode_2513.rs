//! 最小化两个数组中的最大值

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { b } else { gcd(b % a, a) }
}

/// 用4和6把玩一下
pub fn minimize_set(divisor1: i32, divisor2: i32, unique_cnt1: i32, unique_cnt2: i32) -> i32 {
    let (divisor1, divisor2, unique_cnt1, unique_cnt2) = (divisor1 as i64, divisor2 as i64, unique_cnt1 as i64, unique_cnt2 as i64);
    let mut left = 1;
    let mut right = (unique_cnt1 + unique_cnt2) * 2;
    let lcm = divisor1 / gcd(divisor2, divisor1) * divisor2;
    while left < right {
        let mid = (left + right) / 2;
        let left1 = (unique_cnt1 - (mid / divisor2 - mid / lcm)).max(0); // arr1剩余需要选的数 （需要的数 - 独占的数）
        let left2 = (unique_cnt2 - (mid / divisor1 - mid / lcm)).max(0); // arr2剩余需要选的数 （需要的数 - 独占的数）
        let common = mid - (mid / divisor1 + mid / divisor2 - mid / lcm); // 共享的可以选的数：容斥原理（能被4整除的+能被6整除的-能被12整除的）
        if common >= left1 + left2 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as i32
}

fn main() {
    assert_eq!(minimize_set(92761, 48337, 208563424, 9115778), 217679202);
    assert_eq!(minimize_set(2, 7, 1, 3), 4);
    assert_eq!(minimize_set(3, 5, 2, 1), 3);
    assert_eq!(minimize_set(2, 4, 8, 2), 15);
}
