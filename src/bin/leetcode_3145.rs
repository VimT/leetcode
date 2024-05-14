//! 大数组元素的乘积

use leetcode::algorithm::quick_pow;

// 仿照 233 ，写一个统计 0-n 数字中二进制的1的数量
// 原理： 对于 1101 这个数来说，对于第二位 0，假设锁住是1 xx1x ，
//      那么这个数最小可能是 0010，最大可能是 1011，所以这一位的1的个数是 000到101 = 6
// 还有一个结论： [1, 2^(k+1) -1] 区间里， 0-k 位上都含有 2^k 个 1
pub fn count_one_num(n: i64) -> i64 {
    let mut result = 0;
    for i in 0..63 {
        if n >> i == 0 { break; }
        let left = n >> (i + 1);
        let right = n & ((1 << i) - 1);
        let num = n >> i & 1;
        if num == 0 {
            result += left << i;
        } else {
            result += (left << i) + right + 1;
        }
    }
    result
}

// [1, 2, 1,2, 4, 1,4, ...]  这些数都是 2 的幂次方，我们可以统计 1-from 中每个数出现的次数 + 快速幂
// 计算 (0, n) 所有数字的二进制中1的个数：ones(2^k) = k * 2 ^ (k-1)
pub fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
    // 计算 (0,n) 里面 1<<p 出现了几次
    fn count_pos_one(n: i64, i: u32) -> i64 {
        let left = n >> (i + 1);
        let right = n & ((1 << i) - 1);
        let num = n >> i & 1;
        if num == 0 { left << i } else { (left << i) + right + 1 }
    }
    // 计算 (0,n) 里面 1 的个数
    fn count_one(n: i64) -> i64 {
        let mut result = 0;
        for i in 0..64 - n.leading_zeros() {
            result += count_pos_one(n, i);
        }
        result
    }
    // 计算强数组的第x个数所属的数字 k 和 offset
    fn find_pos(x: i64) -> (i64, i64) {
        let mut left = 1;
        let mut right = x + 1;
        while left < right {
            let mid = (left + right) / 2;
            if count_one(mid) < x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        (left, x - count_one(left - 1))
    }

    // 第x个数对应的强数组的 from到to的乘积
    fn calc1(x: i64, from: i64, to: i64, m: i64) -> i64 {
        let mut cur = 0;
        let mut result = 1;
        for i in 0..64 - x.leading_zeros() {
            if x >> i & 1 == 1 {
                cur += 1;
                if cur >= from && cur <= to {
                    result = result * (1 << i) % m;
                }
            }
        }
        result
    }
    // 计算 x1-x2 之间强数组的乘积
    fn calc2(x1: i64, x2: i64, m: i64) -> i64 {
        let mut result = 1;
        for i in 0..64 - x2.leading_zeros() {
            let cnt = count_pos_one(x2, i) - count_pos_one(x1 - 1, i); // x1-x2 之间这一位 1 的个数
            result = (result * quick_pow(1 << i, cnt, m)) % m;
        }
        result
    }

    queries.into_iter().map(|query| {
        let (from, to, mod_num) = (query[0], query[1], query[2]);
        let (from_num, from_offset) = find_pos(from + 1); // 从第 from_num 个数开始，第 from_offset 个数 开始
        let (to_num, to_offset) = find_pos(to + 1); // 到第 to_num 个数，第 to_offset 个数 结束
        if from_num == to_num {
            calc1(from_num, from_offset, to_offset, mod_num)
        } else {
            calc1(from_num, from_offset, 64, mod_num)
                * calc1(to_num, 1, to_offset, mod_num) % mod_num
                * calc2(from_num + 1, to_num - 1, mod_num) % mod_num
        }
    }).map(|x| x as i32).collect()
}

fn main() {
    fn test(func: fn(queries: Vec<Vec<i64>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![2, 5, 3], vec![7, 7, 4]]), vec![2, 2]);
        assert_eq!(func(vec![vec![1, 3, 7]]), vec![4]);
    }
    test(find_products_of_elements);
}
