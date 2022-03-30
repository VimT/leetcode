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

fn main() {
    assert_eq!(num_dup_digits_at_most_n(20), 1);
    assert_eq!(num_dup_digits_at_most_n(100), 10);
    assert_eq!(num_dup_digits_at_most_n(1000), 262);
}
