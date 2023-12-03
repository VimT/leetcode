//! 删除数对后的最小数组长度

pub fn min_length_after_removals(nums: Vec<i32>) -> i32 {
    let mut k = 0;
    let len = nums.len();
    for i in (len + 1) / 2..len {
        if nums[k] < nums[i] {
            k += 1;
        }
    }
    (len - k * 2) as i32
}

pub fn min_length_after_removals2(nums: Vec<i32>) -> i32 {
    let mut mx = 0;
    let mut cur = 1;
    let len = nums.len();
    for w in nums.windows(2) {
        if w[1] == w[0] {
            cur += 1;
        } else {
            mx = mx.max(cur);
            cur = 1;
        }
    }
    mx = mx.max(cur);
    if mx * 2 > len {
        return (mx * 2 - len) as i32;
    }
    (len & 1) as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3]), 4);
        assert_eq!(func(vec![1]), 1);
        assert_eq!(func(vec![1, 1]), 2);
        assert_eq!(func(vec![2, 3, 4, 4, 4]), 1);
        assert_eq!(func(vec![2, 3, 3, 3]), 2);
        assert_eq!(func(vec![2, 3, 4]), 1);
        assert_eq!(func(vec![1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3]), 1);
        assert_eq!(func(vec![1, 1, 4, 4, 5, 5]), 0);
        assert_eq!(func(vec![1, 1, 2, 3, 4, 4]), 0);
        assert_eq!(func(vec![1, 1, 2, 2]), 0);
        assert_eq!(func(vec![2, 3, 4, 4, 4, 4]), 2);
        assert_eq!(func(vec![1, 3, 4, 9]), 0);
        assert_eq!(func(vec![2, 3, 6, 9]), 0);
        assert_eq!(func(vec![1, 1, 2]), 1);
    }
    test(min_length_after_removals);
    test(min_length_after_removals2);
}
