//! 使子序列的和等于目标的最少操作次数

pub fn min_operations(nums: Vec<i32>, target: i32) -> i32 {
    let mut cnt = [0; 33];
    for num in nums {
        cnt[num.trailing_zeros() as usize] += 1;
    }
    let mut result = 0;
    for i in 0..32 {
        if target >> i & 1 == 1 {
            if cnt[i] == 0 {
                for j in i + 1..32 {
                    if cnt[j] > 0 {
                        cnt[j] -= 1;
                        for k in (i..j).rev() { cnt[k] += 1; }
                        result += (j - i) as i32;
                        break;
                    }
                }
            }
            if cnt[i] == 0 { return -1; }
            cnt[i] -= 1;
        }
        cnt[i + 1] += cnt[i] / 2;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 8], 7), 1);
        assert_eq!(func(vec![1, 32, 1, 2], 12), 2);
        assert_eq!(func(vec![1, 32, 1], 35), -1);
    }
    test(min_operations);
}
