//! 数组中两个数的最大异或值

use std::collections::HashSet;

pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    // 依次确定每一位
    for k in (0..=30).rev() {
        let mut seen = HashSet::new();
        // 将所有的 pre^k(a_j) 放入哈希表中
        for &num in &nums {
            // 如果只想保留从最高位开始到第 k 个二进制位为止的部分
            // 只需将其右移 k 位
            seen.insert(num >> k);
        }

        // 目前 x 包含从最高位开始到第 k+1 个二进制位为止的部分
        // 我们将 x 的第 k 个二进制位置为 1，即为 x = x*2+1
        let next = result << 1 | 1;
        let mut found = false;

        // 枚举 i
        for &num in &nums {
            if seen.contains(&(next ^ (num >> k))) {
                found = true;
                break;
            }
        }
        if found {
            result = next;
        } else {
            // 如果没有找到满足等式的 a_i 和 a_j，那么 x 的第 k 个二进制位只能为 0
            // 即为 x = x*2
            result = next - 1;
        }
    }
    result
}

fn main() {
    assert_eq!(find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
    assert_eq!(find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]), 127);
}
