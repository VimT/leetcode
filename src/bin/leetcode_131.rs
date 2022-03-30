//! 分割回文串

pub fn is_palindrome(s: &mut String) -> bool {
    let vec = unsafe { s.as_mut_vec() };
    let mut p1 = 0;
    let mut p2 = vec.len() - 1;
    while p1 < p2 {
        if vec[p1 as usize] != vec[p2 as usize] { return false; }
        p1 += 1;
        p2 -= 1;
    }
    true
}

///递归版
/// 从左到右，依次分隔，左边的如果是回文串，那么左边的加上右边的所有回文串子集，就是结果
pub fn partition(s: String) -> Vec<Vec<String>> {
    fn inner(s: &String) -> Vec<Vec<String>> {
        if 0 == s.len() {
            return vec![vec![]];
        }
        let mut ans: Vec<Vec<String>> = vec![];
        for i in 0..s.len() {
            let mut substr = s[0..=i].to_string();
            if is_palindrome(&mut substr) {
                for mut l in inner(&s[i + 1..s.len()].to_string()) {
                    l.insert(0, substr.clone());
                    ans.push(l);
                }
            }
        }
        return ans;
    }
    inner(&s)
}

///递归->动态规划 优化版
/// 递归过程中，判断是否是回文串有很多重复，bbbb判断依次，abbbba又要判断依次。这种情况只需要判断两边是否相等
/// dp[i][j]表示i-j之间是否是回文串，dp[i][j] = dp[i] == dp[j] && dp[i+1][j-1]
/// 注意从len开始遍历
pub fn partition2(s: String) -> Vec<Vec<String>> {
    let mut dp = vec![vec![false; s.len()]; s.len()];
    let mut ss = s.clone();
    let chars = unsafe { ss.as_mut_vec() };

    for len in 0..s.len() {
        for start in 0..s.len() - len {
            let end = start + len;
            if chars[start] == chars[end] && (len < 3 || dp[start + 1][end - 1]) {
                dp[start][end] = true;
            }
        }
    }

    fn inner(dp: &Vec<Vec<bool>>, s: &String, start: usize) -> Vec<Vec<String>> {
        if s.len() == start {
            let ans = vec![vec![]];
            return ans;
        }
        let mut ans = vec![];
        for i in start..s.len() {
            if dp[start][i] {
                let substr = s[start..i + 1].to_string();
                for mut l in inner(dp, s, i + 1) {
                    l.insert(0, substr.clone());
                    ans.push(l);
                }
            }
        }
        ans
    }

    inner(&dp, &s, 0)
}

/// 深度优先遍历，回溯法
pub fn partition_dfs(s: String) -> Vec<Vec<String>> {
    let mut dp = vec![vec![false; s.len()]; s.len()];
    let s = s.as_bytes();
    let len = s.len();
    // i mean current dp length
    for i in 0..len {
        for start in 0..len - i {
            let end = start + i;
            dp[start][end] = s[start] == s[end] && (i < 3 || dp[start + 1][end - 1]);
        }
    }
    fn inner(s: &[u8], start: usize, current_vec: &mut Vec<String>, ans: &mut Vec<Vec<String>>, dp: &Vec<Vec<bool>>) {
        if start == s.len() {
            ans.push(current_vec.clone());
        }
        for i in start..s.len() {
            if dp[start][i] {
                let substr = unsafe { String::from_utf8_unchecked(s[start..i + 1].to_vec()) };
                current_vec.push(substr);
                inner(s, i + 1, current_vec, ans, dp);
                current_vec.pop();
            }
        }
    }
    let mut current_vec: Vec<String> = vec![];
    let mut ans: Vec<Vec<String>> = vec![];
    inner(s, 0, &mut current_vec, &mut ans, &dp);
    ans
}

pub fn partition_magic(s: String) -> Vec<Vec<String>> {
    let mut ans: Vec<Vec<String>> = vec![vec![s[0..1].to_string()]];
    for letter in s[1..s.len()].chars() {
        let letter_s = letter.to_string();
        for i in 0..ans.len() {
            let item = &ans[i];
            // 当前字母和回文串列表倒数第二个字符串相同时，就可以产生新的结果， ['a', 'bb'] & 'a' => ['abba']
            let mut tmp1: Vec<String> = vec![];
            let mut tmp2: Vec<String> = vec![];
            if item.len() > 1 && letter_s == item[item.len() - 2] {
                tmp1.extend(item[0..item.len() - 2].to_vec());
                tmp1.push(format!("{}{}{}", item[item.len() - 2], item.last().unwrap(), letter_s));
            }
            // 当前字母 和 回文串列表里的最后一个 字符串相同时，可以产生新的结果  ['abba', 'b'] & 'b' => ['abba', 'bb']
            if letter_s == *item.last().unwrap() {
                tmp2.extend(item[0..item.len() - 1].to_vec());
                tmp2.push(format!("{}{}", letter_s, letter_s));
            }
            if tmp1.len() > 0 { ans.push(tmp1); }
            if tmp2.len() > 0 { ans.push(tmp2); }
            ans[i].push(letter_s.clone());
        }
    }
    ans
}


fn main() {
    fn test(func: fn(s: String) -> Vec<Vec<String>>) {
        assert_eq!(func(String::from("aab")), vec![vec!["a", "a", "b"], vec!["aa", "b"]]);
        assert_eq!(func(String::from("a")), vec![vec!["a"]]);
    }
    test(partition);
    test(partition2);
    test(partition_dfs);
    test(partition_magic);
}
