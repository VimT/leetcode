//! 阶乘函数后 K 个零

#[inline]
pub fn trailing_zeroes(mut n: u32) -> u32 {
    let mut ans = 0;
    while n >= 5 {
        n = n / 5;
        ans += n;
    }
    ans
}

pub fn preimage_size_fzf(k: i32) -> i32 {
    let mut left = 0;
    let mut right = 1e10 as u32;
    let k = k as u32;
    while left < right {
        let mid = left + (right - left) / 2;
        let cnt = trailing_zeroes(mid);
        if cnt == k { return 5; }
        if cnt > k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    0
}

fn main() {
    assert_eq!(preimage_size_fzf(1000000000), 5);
    assert_eq!(preimage_size_fzf(0), 5);
    assert_eq!(preimage_size_fzf(5), 0);
    assert_eq!(preimage_size_fzf(3), 5);
}
