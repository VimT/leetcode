//! 最小无法得到的或值

pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
    let mut has = [false; 32];
    for num in nums {
        if num.count_ones() == 1 {
            has[num.trailing_zeros() as usize] = true;
        }
    }
    for i in 0..32 {
        if !has[i] {
            return 1 << i;
        }
    }
    unreachable!()
}

/// 全部或之后找最后一个0
pub fn min_impossible_or2(nums: Vec<i32>) -> i32 {
    let or = !nums.into_iter().fold(0, |or, x| if x & (x - 1) == 0 { x | or } else { or });
    or & -or
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 1]), 4);
        assert_eq!(func(vec![5, 3, 2]), 1);
    }
    test(min_impossible_or);
    test(min_impossible_or2);
}
