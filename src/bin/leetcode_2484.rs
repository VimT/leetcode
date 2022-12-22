//! 统计回文子序列数目


pub fn count_palindromes(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut left = vec![[0; 100]; len];  // 到i为止，XX出现的次数
    let mut left_num = [0; 10];
    for i in 0..len {
        let idx = (s[i] - b'0') as usize;
        if i > 0 { left[i] = left[i - 1].clone(); }
        for ten in 0..10 {
            left[i][ten * 10 + idx] += left_num[ten];
        }
        left_num[idx] += 1;
    }
    let mut right_num = [0; 10];
    let mut right = [0; 100];
    let mut result = 0;
    const MOD: i64 = 1e9 as i64 + 7;
    for i in (2..len).rev() {
        let idx = (s[i] - b'0') as usize;
        let mut new_right = right.clone();
        for ten in 0..10 {
            new_right[ten * 10 + idx] += right_num[ten];
        }

        if i < len - 2 {
            for xx in 0..100 {
                result = (result + left[i - 1][xx] * right[xx]) % MOD;
            }
        }
        right_num[idx] += 1;
        right = new_right;
    }
    result as i32
}

fn main() {
    assert_eq!(count_palindromes(String::from("103301")), 2);
    assert_eq!(count_palindromes(String::from("0000000")), 21);
    assert_eq!(count_palindromes(String::from("9999900000")), 2);
}
