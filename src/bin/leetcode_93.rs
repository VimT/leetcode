//! 复原 IP 地址

use leetcode::svec;

pub fn restore_ip_addresses(s: String) -> Vec<String> {
    fn inner(s: &String, index: usize, current: &mut Vec<String>, ans: &mut Vec<String>) {
        if index == s.len() && current.len() == 4 {
            ans.push(current.join("."));
        }
        if index == s.len() || current.len() > 4 { return; }
        for end in index + 1..=(index + 3).min(s.len()) {
            let v = s[index..end].to_string();
            let int_v = v.parse::<i32>().unwrap();
            if int_v > 255 { continue; }
            current.push(v);
            inner(s, end, current, ans);
            if current.pop().unwrap() == "0" { break; }
        }
    }
    let mut ans = vec![];
    inner(&s, 0, &mut vec![], &mut ans);
    ans
}

fn main() {
    assert_eq!(restore_ip_addresses(String::from("25525511135")), svec!["255.255.11.135", "255.255.111.35"]);
    assert_eq!(restore_ip_addresses(String::from("0000")), svec!["0.0.0.0"]);
    assert_eq!(restore_ip_addresses(String::from("101023")), svec!["1.0.10.23", "1.0.102.3", "10.1.0.23", "10.10.2.3", "101.0.2.3"]);
}
