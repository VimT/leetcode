//! 字符串相加

pub fn add_strings(num1: String, num2: String) -> String {
    let n1 = num1.as_bytes();
    let n2 = num2.as_bytes();
    let len = n1.len().max(n2.len());
    let mut p1 = n1.len();
    let mut p2 = n2.len();
    let mut ans = vec![b'0'; len];
    let mut carry = 0;
    for i in 0..len {
        let mut num = carry;
        if p1 > 0 {
            num += n1[p1 - 1] - b'0';
            p1 -= 1;
        }
        if p2 > 0 {
            num += n2[p2 - 1] - b'0';
            p2 -= 1;
        }
        ans[i] = b'0' + num % 10;
        carry = num / 10;
    }
    if carry > 0 {
        ans.push(b'0' + carry);
    }
    ans.reverse();
    String::from_utf8(ans).unwrap()
}

fn main() {
    fn test(func: fn(num1: String, num2: String) -> String) {
        assert_eq!(func(String::from("11"), String::from("123")), String::from("134"));
        assert_eq!(func(String::from("456"), String::from("77")), String::from("533"));
        assert_eq!(func(String::from("0"), String::from("0")), String::from("0"));
    }
    test(add_strings);
}
