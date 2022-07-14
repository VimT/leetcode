//! 第一个错误的版本

struct Solution {
    start_bad: i32,
}

impl Solution {
    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        return version >= self.start_bad;
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 0;
        let mut right = n;
        while left < right {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}


fn main() {
    let help = |n: i32, start_bad: i32| {
        let s = Solution { start_bad };
        return s.first_bad_version(n);
    };
    assert_eq!(help(5, 4), 4);
    assert_eq!(help(1, 1), 1);
}
