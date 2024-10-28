//! 使数组非递减的最少除法操作次数

use lazy_static::lazy_static;

pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    lazy_static! {
        // 最小质因子
        static ref lpf: Vec<i32> = {
            const MX: usize = 1_000_001;
            let mut tmp = vec![0; MX];
            for i in 2..MX {
                if tmp[i] == 0 {
                    let mut j = i;
                    while j < MX {
                        if tmp[j] == 0 { tmp[j] = i as i32; }
                        j += i;
                    }
                }
            }
            tmp
        };
    }
    let mut result = 0;
    let len = nums.len();
    for i in (0..len - 1).rev() {
        if nums[i] > nums[i + 1] {
            nums[i] = lpf[nums[i] as usize];
            if nums[i] > nums[i + 1] { return -1; }
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![25, 7]), 1);
        assert_eq!(func(vec![7, 7, 6]), -1);
        assert_eq!(func(vec![1, 1, 1, 1]), 0);
    }
    test(min_operations);
}
