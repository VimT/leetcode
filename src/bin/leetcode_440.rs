//! 字典序的第K小数字

pub fn find_kth_number(n: i32, k: i32) -> i32 {
    #[inline]
    fn prefix_cnt(prefix: i64, n: i64) -> i64 {
        let mut cur = prefix;
        let mut next = prefix + 1;
        let mut count = 0;
        while cur <= n {
            count += next.min(n + 1) - cur;
            cur *= 10;
            next *= 10;
        }
        count
    }
    let mut prefix = 1;
    let mut count = 1;
    let k = k as i64;
    while count < k {
        let cur_count = prefix_cnt(prefix, n as i64);
        if count + cur_count > k {
            prefix *= 10;
            count += 1;
        } else if count + cur_count <= k {
            prefix += 1;
            count += cur_count;
        }
    }
    prefix as i32
}

fn main() {
    assert_eq!(find_kth_number(681692778, 351251360), 416126219);
    assert_eq!(find_kth_number(13, 2), 10);
    assert_eq!(find_kth_number(1, 1), 1);
}
