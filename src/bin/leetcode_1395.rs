//! 统计作战单位数

pub fn num_teams(rating: Vec<i32>) -> i32 {
    let len = rating.len();
    let mut more = vec![0; len];
    let mut less = vec![0; len];
    for (i, num) in rating.iter().enumerate() {
        for num2 in &rating[i + 1..] {
            if num2 > num {
                more[i] += 1;
            } else {
                less[i] += 1;
            }
        }
    }
    let mut result = 0;
    for (i, num) in rating.iter().enumerate() {
        for (j, num2) in rating[i + 1..].iter().enumerate() {
            if num > num2 {
                result += less[j + i + 1];
            } else  {
                result += more[j + i + 1];
            }
        }
    }
    result
}

/// 巧妙的一次遍历，同时维护两个dp
pub fn num_teams2(rating: Vec<i32>) -> i32 {
    let len = rating.len();
    let mut dp1 = vec![0; len]; // 左边有多少个数比i小
    let mut dp2 = vec![0; len]; // 左边有多少个数比i大
    let mut ans = 0;
    for (idx, &val) in rating.iter().enumerate() {
        for j in 0..idx {
            if rating[j] < val {
                ans += dp1[j];
                dp1[idx] += 1;
            } else {
                ans += dp2[j];
                dp2[idx] += 1;
            }
        }
    }
    ans
}

fn main() {
    fn test(func: fn(rating: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 5, 3, 4, 1]), 3);
        assert_eq!(func(vec![2, 1, 3]), 0);
        assert_eq!(func(vec![1, 2, 3, 4]), 4);
    }
    test(num_teams);
    test(num_teams2);
}
