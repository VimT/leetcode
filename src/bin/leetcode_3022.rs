//! 给定操作次数内使剩余元素的或值最小

/// 在考虑每一位最终是否为零时，怎么保存住之前那些确定最终结果可以为0的位。使用mask 巧妙地解决了这个问题
pub fn min_or_after_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;
    let mut mask = 0;
    let mx = nums.iter().max().copied().unwrap();
    let bit_len = 32 - mx.leading_zeros();
    for b in (0..bit_len).rev() {
        mask |= 1 << b;
        let mut cnt = 0; // 操作次数
        let mut and = -1;
        for &x in &nums {
            and &= x & mask;
            if and > 0 {
                cnt += 1; // 合并 x，操作次数+1
            } else {
                and = -1;
            }
        }
        if cnt > k {
            result |= 1 << b; // 答案的这个比特位必须是1
            mask ^= 1 << b; // 后面不考虑这个比特位
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![3, 5, 3, 2, 7], 2), 3);
        assert_eq!(func(vec![7, 3, 15, 14, 2, 8], 4), 2);
        assert_eq!(func(vec![10, 7, 10, 3, 9, 14, 9, 4], 1), 15);
    }
    test(min_or_after_operations);
}
