//! 数字 1 的个数


pub fn count_digit_one(n: i32) -> i32 {
    let n = n as i64;
    let mut ans = 0;
    let mut div = 1;
    while n / div > 0 {
        let left = n / div / 10;
        let right = n % div;
        let num = n / div % 10;
        if num == 0 {
            ans += left * div;
        } else if num == 1 {
            ans += left * div + right + 1;
        } else {
            ans += (left + 1) * div;
        }
        div *= 10;
    }
    ans as i32
}


// 数位DP模板
fn count_digit_one2(n: i32) -> i32 {
    // i: 当前枚举到多少位，cnt: 前i位有多少个 1
    fn dfs(n: &Vec<u8>, i: usize, cnt: usize, is_limit: bool, mem: &mut Vec<Vec<usize>>) -> usize {
        if i == n.len() {
            return cnt;
        }
        if !is_limit && mem[i][cnt] != 0 {
            return mem[i][cnt];
        }
        let up = if is_limit { n[i] } else { 9 };
        let mut result = 0;
        for j in 0..=up {
            result += dfs(n, i + 1, cnt + (j == 1) as usize, is_limit && j == up, mem);
        }
        if !is_limit { mem[i][cnt] = result; }
        result
    }
    let v = n.to_string().as_bytes().iter().map(|&x| (x - b'0')).collect::<Vec<u8>>();
    let mut mem = vec![vec![0; v.len() + 1]; v.len()];
    dfs(&v, 0, 0, true, &mut mem) as i32
}


fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(20), 12);
        assert_eq!(func(13), 6);
        assert_eq!(func(0), 0);
    }
    test(count_digit_one);
    test(count_digit_one2);
}
