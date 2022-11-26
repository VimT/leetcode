//! 根据限制分割消息

pub fn split_message(message: String, limit: i32) -> Vec<String> {
    let s = message.as_bytes();
    let len = s.len();
    fn cal(mut n: usize) -> usize {
        let mut str_len = n.to_string().as_bytes().len();
        let mut result = str_len * n + 3 * n;
        while n > 9 {
            let pow = 10usize.pow((str_len - 1) as u32);
            result += str_len * (n + 1 - pow);
            n = pow - 1;
            str_len -= 1;
        }
        result += n;
        result
    }

    let limit = limit as usize;
    let mut left = 1;
    let mut right = s.len() + 1;
    while left < right {
        let lines = (left + right) / 2;
        let real_s_len = len + cal(lines);
        let mut real_lines = real_s_len / limit;
        if real_lines * limit < real_s_len {
            real_lines += 1;
        }
        if real_lines <= lines {
            right = lines;
        } else {
            left = lines + 1;
        }
    }
    let line = left;
    if line > len { return vec![]; }
    let line_str_len = line.to_string().as_bytes().len();
    let mut i = 0;
    let mut result = Vec::with_capacity(line + 1);
    let mut line_idx = 1;
    while i < len {
        let line_len = limit - 3 - line_str_len - line_idx.to_string().len();
        result.push(format!("{}<{}/{}>", &message[i..(i + line_len).min(len)], line_idx, line));
        i += line_len;
        line_idx += 1;
    }
    result
}

pub fn split_message2(message: String, limit: i32) -> Vec<String> {
    let len = message.len();
    let mut i = 0;
    let mut cap = 0;
    // 注意这种 i-cap 图不是单调递增的，不可以二分
    loop {
        i += 1;
        let tail_len = if i < 10 {
            5
        } else if i < 100 {
            if i == 10 { cap -= 9; }
            7
        } else if i < 1000 {
            if i == 100 { cap -= 99; }
            9
        } else {
            if i == 1000 { cap -= 999; }
            11
        };
        if limit - tail_len <= 0 { return vec![]; }
        cap += limit - tail_len;
        if cap < len as i32 { continue; }

        let mut result = Vec::with_capacity(i);
        let mut mi = 0;
        for j in 1..=i {
            let x = format!("<{}/{}>", j, i);
            let l = limit as usize - x.len();
            result.push(format!("{}{}", &message[mi..(mi + l).min(len)], x));
            mi += l;
        }
        return result
    }
}

fn main() {
    fn test(func: fn(message: String, limit: i32) -> Vec<String>) {
        assert_eq!(func(String::from("this is really a very awesome message"), 9), vec!["thi<1/14>", "s i<2/14>", "s r<3/14>", "eal<4/14>", "ly <5/14>", "a v<6/14>", "ery<7/14>", " aw<8/14>", "eso<9/14>", "me<10/14>", " m<11/14>", "es<12/14>", "sa<13/14>", "ge<14/14>"]);
        assert_eq!(func(String::from("short message"), 15), vec!["short mess<1/2>", "age<2/2>"]);
    }
    test(split_message);
    test(split_message2);
}
