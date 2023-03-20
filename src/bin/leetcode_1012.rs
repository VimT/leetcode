//! 至少有 1 位重复的数字


pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
    let mut limit = vec![];
    let mut num = n;
    while num > 0 {
        limit.push((num % 10) as u8);
        num /= 10;
    }
    limit.reverse();
    let mut factorial = vec![0; 10];
    factorial[0] = 1;
    for i in 1..10 {
        factorial[i as usize] = i * factorial[i as usize - 1];
    }
    let a = |m: usize, n: usize| -> i32 {
        factorial[m] / factorial[m - n]
    };
    let len = limit.len();
    let mut cnt = 0;
    let mut used = [false; 10];
    // 算所有位填充不同数有多少种
    // 长度为i时，第一位有9种填法，剩下的位是A(9, i-1)
    for i in 1..=len {
        cnt += 9 * a(9, i - 1);
    }
    // 再减去 多出来的那部分
    for i in 0..len {
        let num = limit[i] as usize;
        // 从N的首位开始 如果该位为a 则从a+1到9 都可以放 然后后面n-1位0-10随便取n-1个做全排列
        for j in num + 1..10 {
            if !used[j] {
                cnt -= a(9 - i, len - 1 - i);
            }
        }
        // 如果发现这一位的值之前已经出现过 比如1000 十位上的0和之前存在dup里的百位的0重复了 后续不会再有不重复的数字 退出
        if used[num] {
            break;
        }
        used[num] = true;
    }
    n - cnt
}

/// 数位DP
/// 转换成求无重复数字的个数
pub fn num_dup_digits_at_most_n2(n: i32) -> i32 {
    struct DFS {
        cache: Vec<Vec<[[i32; 2]; 2]>>,
        s: Vec<u8>,

    }
    impl DFS {
        /// f(i,mask,isLimit,isNum) 表示构造第 i 位及其之后数位的合法方案数
        /// isLimit 表示当前是否受到了 n 的约束
        /// mask 表示前面选过的数字集合，换句话说，第 i 位要选的数字不能在 mask 中
        /// isNum 表示 i 前面的数位是否填了数字，如果true表示填入的数字可以从0开始，如果为false表示可以跳过这个数
        fn dfs(&mut self, i: usize, mask: usize, is_limit: bool, is_num: bool) -> i32 {
            if i == self.s.len() { return is_num as i32; }
            if self.cache[i][mask][is_limit as usize][is_num as usize] != -1 {
                return self.cache[i][mask][is_limit as usize][is_num as usize];
            }
            let mut result = 0;
            if !is_num {
                result += self.dfs(i + 1, mask, false, false);
            }
            let start = if is_num { 0 } else { 1 };
            let end = if is_limit { self.s[i] } else { 9 };
            for num in start..=end {
                if mask >> num & 1 == 0 {
                    result += self.dfs(i + 1, mask | 1 << num, is_limit && num == self.s[i], true);
                }
            }
            self.cache[i][mask][is_limit as usize][is_num as usize] = result;
            result
        }
    }
    let mut s = n.to_string().into_bytes();
    for n in &mut s {
        *n -= b'0';
    }
    let len = s.len();
    let mut d = DFS { cache: vec![vec![[[-1; 2]; 2]; 1 << 10]; len], s };
    n - d.dfs(0, 0, true, false)
}


fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(20), 1);
        assert_eq!(func(100), 10);
        assert_eq!(func(1000), 262);
    }
    test(num_dup_digits_at_most_n);
    test(num_dup_digits_at_most_n2);
}
