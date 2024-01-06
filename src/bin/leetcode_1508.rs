//! 子数组和排序后的区间和

pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
    let n = n as usize;
    let mut all = Vec::with_capacity(n * (n + 1) / 2);
    for i in 0..n {
        let mut sum = 0;
        for j in i..n {
            sum += nums[j];
            all.push(sum);
        }
    }
    const MOD: i32 = 1e9 as i32 + 7;
    all.sort_unstable();
    all[left as usize - 1..right as usize].iter().fold(0, |acc, x| (acc + *x) % MOD)
}


/// 可以把nums的前缀和转化成一个有序矩阵：
/// 前缀和矩阵 S，其在第 i 行第 j 列的元素 s[i][j] 为 nums 的从第 i 个元素到第 j 个元素的和。
/// 用类似378题的思路 求有序矩阵第k小的数
/// 复杂度 nlogn
pub fn range_sum2(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
    let n = n as usize;
    let mut ps = vec![0; n + 1];
    for i in 0..n {
        ps[i + 1] = ps[i] + nums[i] as i64;
    }
    let mut pps = vec![0; n + 1];
    for i in 0..n {
        pps[i + 1] = pps[i] + ps[i + 1];
    }
    // <x 的所有区间和 的和 sum 和 个数 cnt
    let get_sum_cnt = |x: i64| -> (i64, i64) {
        let mut cnt = 0;
        let mut sum = 0;
        let mut j = 0;
        for i in 0..n as i64 {
            while j < n as i64 && ps[j as usize + 1] - ps[i as usize] < x {
                j += 1;
            }
            sum += pps[j as usize] - pps[i as usize] - (j - i) * ps[i as usize];
            cnt += j - i;
        }
        (sum, cnt)
    };
    let get_kth = |k: i64| -> i64 {
        let mut left = 0;
        let mut right = ps[n];
        while left < right {
            let mid = (left + right + 1) / 2;
            let (_, cnt) = get_sum_cnt(mid);
            if cnt < k {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    };
    let f = |k: i64| -> i64 {
        let x = get_kth(k);
        let (sum, cnt) = get_sum_cnt(x);
        sum + (k - cnt) * x
    };
    ((f(right as i64) - f((left - 1) as i64)) % (1e9 as i64 + 7)) as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32) {
        assert_eq!(func(vec![7, 5, 8, 5, 6, 4, 3, 3], 8, 2, 6), 23);
        assert_eq!(func(vec![1, 2, 3, 4], 4, 1, 5), 13);
        assert_eq!(func(vec![1, 2, 3, 4], 4, 3, 4), 6);
        assert_eq!(func(vec![1, 2, 3, 4], 4, 1, 10), 50);
    }
    test(range_sum);
    test(range_sum2);
}
