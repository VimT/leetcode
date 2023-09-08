//! 操作使得分最大

use leetcode::algorithm::quick_pow;

const MAX: usize = 100001;
static mut SCORE: [i32; MAX] = [0; MAX];

unsafe fn init() {
    if SCORE[2] == 0 {
        for i in 2..MAX {
            if SCORE[i] == 0 {
                let mut j = i;
                while j < MAX {
                    SCORE[j] += 1;
                    j += i;
                }
            }
        }
    }
}

pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    unsafe { init(); }
    let prime_score: Vec<i32> = nums.iter().copied().map(|x| unsafe { SCORE[x as usize] }).collect();
    let len = nums.len();
    let mut left = vec![-1; len]; // 前面第一个 >= 这个数 + 1
    let mut right = vec![len as i64; len];  // 后面第一个 > 这个数
    let mut s = vec![];
    for i in 0..len {
        while !s.is_empty() && prime_score[*s.last().unwrap()] < prime_score[i] {
            right[s.pop().unwrap()] = i as i64;
        }
        s.push(i);
    }
    s.clear();
    for i in (0..len).rev() {
        while !s.is_empty() && prime_score[*s.last().unwrap()] <= prime_score[i] {
            left[s.pop().unwrap()] = i as i64;
        }
        s.push(i);
    }
    let mut nums: Vec<(i32, i64)> = nums.into_iter().zip(left.into_iter().zip(right).zip(0..).map(|((l, r), i)| (r - i) * (i - l))).collect();
    nums.sort_unstable();
    let mut result = 1;
    const MOD: i64 = 1e9 as i64 + 7;
    let mut k = k as i64;
    for (num, cnt) in nums.into_iter().rev() {
        result *= quick_pow(num as i64, cnt.min(k), MOD);
        result %= MOD;
        k -= cnt.min(k);
        if k == 0 { break; }
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![3289, 2832, 14858, 22011], 6), 256720975);
        assert_eq!(func(vec![8, 3, 9, 3, 8], 2), 81);
        assert_eq!(func(vec![19, 12, 14, 6, 10, 18], 3), 4788);
    }
    test(maximum_score);
}
