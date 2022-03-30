//! 为运算表达式设计优先级

use leetcode::unorder;

pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
    fn inner(input: &[u8]) -> Vec<i32> {
        let mut only_digital = true;
        let mut ans = vec![];
        for (i, &char) in input.iter().enumerate() {
            if char == b'+' || char == b'-' || char == b'*' {
                only_digital = false;
                let left = inner(&input[0..i]);
                let right = inner(&input[i + 1..]);
                for l in left {
                    for &r in &right {
                        match char {
                            b'+' => ans.push(l + r),
                            b'-' => ans.push(l - r),
                            b'*' => ans.push(l * r),
                            _ => panic!("should never happen")
                        }
                    }
                }
            }
        }
        if only_digital {
            let mut num = 0;
            for &i in input {
                num = num * 10 + (i - b'0') as i32;
            }
            ans.push(num);
        }
        ans
    }
    return inner(input.as_bytes());
}


fn main() {
    assert_eq!(unorder(diff_ways_to_compute(String::from("2-1-1"))), vec![0, 2]);
    assert_eq!(unorder(diff_ways_to_compute(String::from("2*3-4*5"))), vec![-34, -14, -10, -10, 10]);
}
