//! 使数组成为等数数组的最小代价

pub fn minimum_cost(mut nums: Vec<i32>) -> i64 {
    let len = nums.len();
    nums.sort_unstable();
    let mid = nums[len / 2] as i64;
    let mut result = i64::MAX;
    fn is_palindrome(x: i64) -> bool {
        let x = x.to_string();
        let len = x.len();
        x.as_bytes()[..len / 2].iter().zip(x.as_bytes()[len / 2..].iter().rev()).all(|(&a, &b)| a == b)
    }
    if is_palindrome(mid) {
        result = nums.iter().map(|&num| (num as i64 - mid).abs()).sum::<i64>();
    }
    for i in (0..mid).rev() {
        if is_palindrome(i) {
            result = result.min(nums.iter().map(|&num| (num as i64 - i).abs()).sum::<i64>());
            break;
        }
    }
    for i in mid + 1.. {
        if is_palindrome(i) {
            result = result.min(nums.iter().map(|&num| (num as i64 - i).abs()).sum::<i64>());
            break;
        }
    }

    result
}

/// 预处理所有回文数
pub fn minimum_cost2(mut nums: Vec<i32>) -> i64 {
    let palindrome_nums = unsafe {
        static mut P: Vec<i32> = Vec::new();
        if P.is_empty() {
            let mut base = 1;
            while base <= 10000 {
                // 奇数长度的回文数
                for i in base..base * 10 {
                    let mut x = i;
                    let mut y = i / 10;
                    while y > 0 {
                        x = x * 10 + y % 10;
                        y /= 10;
                    }
                    P.push(x);
                }
                // 偶数长度的回文数
                if base <= 1000 {
                    for i in base..base * 10 {
                        let mut x = i;
                        let mut y = i;
                        while y > 0 {
                            x = x * 10 + y % 10;
                            y /= 10;
                        }
                        P.push(x);
                    }
                }
                base *= 10;
            }
            P.push(1_000_000_001); // 哨兵，防止越界
        }
        &P
    };
    nums.sort_unstable();
    let len = nums.len();
    let mid = nums[(len - 1) / 2];  // 左侧的中位数
    let i = palindrome_nums.binary_search_by(|x| x.cmp(&mid).then(std::cmp::Ordering::Greater)).unwrap_err();
    let cost = |target: i32| -> i64 {
        nums.iter().map(|&num| (num as i64 - target as i64).abs()).sum::<i64>()
    };
    if palindrome_nums[i] <= nums[len / 2] {
        return cost(palindrome_nums[i]);
    }
    cost(palindrome_nums[i - 1]).min(cost(palindrome_nums[i]))
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![150, 722, 102, 628, 272, 539, 753, 161, 814, 930]), 2623);
        assert_eq!(func(vec![101, 102, 105, 108, 124]), 35);
        assert_eq!(func(vec![1, 3, 4]), 3);
        assert_eq!(func(vec![1, 2, 3, 4, 5]), 6);
        assert_eq!(func(vec![10, 12, 13, 14, 15]), 11);
        assert_eq!(func(vec![22, 33, 22, 33, 22]), 22);
    }
    test(minimum_cost);
    test(minimum_cost2);
}
