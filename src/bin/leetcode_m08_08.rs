//! 面试题 08.08. 有重复字符串的排列组合

pub fn permutation(s: String) -> Vec<String> {
    let s = s.as_bytes().to_vec();
    let len = s.len();
    let mut ans = vec![s];

    for position in 0..len - 1 {
        for i in 0..ans.len() {
            ans[i][position..].sort_unstable();
            for j in position + 1..len {
                let mut tmp = ans[i].clone();
                if tmp[j] == tmp[j - 1] { continue; }
                tmp.swap(position, j);
                ans.push(tmp);
            }
        }
    }
    unsafe { ans.into_iter().map(|x| String::from_utf8_unchecked(x)).collect() }
}

fn main() {
    println!("{:?}", permutation("qqe".to_string()));
    println!("{:?}", permutation("ab".to_string()));
}
