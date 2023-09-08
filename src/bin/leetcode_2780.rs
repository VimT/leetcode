//! 合法分割的最小下标

pub fn minimum_index(nums: Vec<i32>) -> i32 {
    let mut cnt = 0;
    let mut zp = 0;
    for &num in &nums {
        if cnt == 0 {
            zp = num;
        }
        cnt += if num == zp { 1 } else { -1 };
    }

    let cnt = nums.iter().filter(|&&x| x == zp).count();
    let len = nums.len();
    let mut left_cnt = 0;
    for i in 0..len - 1 {
        if nums[i] == zp { left_cnt += 1; }
        if left_cnt * 2 > (i + 1) && (cnt - left_cnt) * 2 > (len - i - 1) {
            return i as i32;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 2, 2]), 2);
        assert_eq!(func(vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1]), 4);
        assert_eq!(func(vec![3, 3, 3, 3, 7, 2, 2]), -1);
    }
    test(minimum_index);
}
