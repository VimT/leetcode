//! 得到回文串的最少操作次数

/// 贪心，对左边的每个字符，找离右边最近的。如果是只出现一次的，就放在中间复杂度O(n2)
/// 更复杂做法：通过双端队列维护每个字母出现的位置，并用树状数组维护哪些位置的字母已经被交换出去。复杂度O(nlogn)。
pub fn min_moves_to_make_palindrome(mut s: String) -> i32 {
    let s = unsafe { s.as_bytes_mut() };
    let len = s.len();
    let mut result = 0;
    let mut i = 0;
    let mut j = len - 1;
    while i < j {
        let mut found = false;
        for k in (i + 1..=j).rev() {
            if s[i] == s[k] {
                found = true;
                // 字母出现偶数次的情况，模拟交换
                for l in k..j {
                    s.swap(l, l + 1);
                    result += 1;
                }
                j -= 1;
                break;
            }
        }
        if !found {
            // 字母出现奇数次的情况，计算它距离中间还有多少距离
            result += (len / 2 - i) as i32;
        }
        i += 1;
    }
    result
}

fn main() {
    assert_eq!(min_moves_to_make_palindrome(String::from("aabbc")), 4);
    assert_eq!(min_moves_to_make_palindrome(String::from("aabb")), 2);
    assert_eq!(min_moves_to_make_palindrome(String::from("letelt")), 2);
}
