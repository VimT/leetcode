//! 寻找重复数


/// 快慢指针
/// 假设环长为 L，从起点到环的入口的步数是 a，从环的入口继续走 b 步到达相遇位置，从相遇位置继续走 c 步回到环的入口，则有 b+c=L
/// 2(a+b)=a+b+kL
/// a=(k−1)L+(L−b)=(k−1)L+c
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut fast = 0;
    let mut slow = 0;
    loop {
        fast = nums[fast as usize];
        fast = nums[fast as usize];
        slow = nums[slow as usize];
        if fast == slow {
            break;
        }
    }
    slow = 0;
    loop {
        fast = nums[fast as usize];
        slow = nums[slow as usize];
        if fast == slow {
            break;
        }
    }

    return fast;
}

/// 二分查找
/// cnt[i] 表示 <= i 的数有多少个
pub fn find_duplicate_bin_search(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    // l,r 表示数字的左右不分
    let mut l: i32 = 1;
    let mut r: i32 = (len - 1) as i32;
    let mut ans = -1;
    while l <= r {
        let mid = l + (r - l) / 2;
        let cnt = nums.iter().filter(|x| **x <= mid).count() as i32;
        if cnt <= mid {
            l = mid + 1;
        } else {
            r = mid - 1;
            ans = mid;
        }
    }
    ans
}

/// 位运算。。神奇
/// 题目可以理解为，[1,n] 中有个数被替换为 ans
/// 分析二进制性质可以发现，替换之后  ans对应位数为1的地方，nums的对应位1的数量 > [1,n)对应位1的数量
/// 证明：
/// 如果被替换的数第 i 位为 1，且 target 第 i 位为 1：x 不变，满足 x>y
/// 如果被替换的数第 i 位为 0，且 target 第 i 位为 1：x 加一，满足 x>y
/// 如果被替换的数第 i 位为 1，且 target 第 i 位为 0：x 减一，满足 x≤y。
/// 如果被替换的数第 i 位为 0，且 target 第 i 位为 0：x 不变，满足 x≤y。
pub fn find_duplicate_bit(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut ans = 0;
    let mut bit_max = 31;
    while (len - 1 >> bit_max) == 0 {
        bit_max -= 1;
    }
    for bit in 0..=bit_max {
        let mut x = 0;
        let mut y = 0;
        for i in 0..len {
            if (nums[i] & (1 << bit)) != 0 { x += 1; }
            if i >= 1 && (i & (1 << bit)) != 0 { y += 1; }
        }
        if x > y {
            ans |= 1 << bit;
        }
    }

    ans
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(func(vec![3, 1, 3, 4, 2]), 3);
    }
    test(find_duplicate);
    test(find_duplicate_bin_search);
    test(find_duplicate_bit);
}
