//! 得到 K 个半回文串的最少修改次数

pub fn minimum_changes(s: String, k: i32) -> i32 {
    fn get_modify(s: &[u8]) -> i32 {
        let divisor = unsafe {
            const MAX: usize = 200;
            static mut DIVISOR: Vec<Vec<usize>> = vec![];
            if DIVISOR.is_empty() {
                DIVISOR = vec![vec![]; MAX + 1];
                for i in 1..=MAX {
                    let mut j = i * 2;
                    while j <= MAX {
                        DIVISOR[j].push(i);
                        j += i;
                    }
                }
            }
            &DIVISOR
        };
        let mut result = s.len() as i32;
        let len = s.len();
        for &d in &divisor[len] {
            let mut cnt = 0;
            for start in 0..d {
                let mut i = start;
                let mut j = len + start - d;
                while i < j {
                    cnt += (s[i] != s[j]) as i32;
                    i += d;
                    j -= d;
                }
            }
            result = result.min(cnt);
        }
        result
    }

    let s = s.as_bytes();
    let len = s.len();

    let mut dp1 = vec![vec![0; len]; len];  // dp1[i][j] 表示把 s[i..=j] 变成半回文串需要的最少修改次数
    for i in 0..len - 1 {
        for j in i + 1..len {
            dp1[i][j] = get_modify(&s[i..=j]);
        }
    }
    let k = k as usize;
    let mut dp2 = vec![vec![i32::MAX / 2; k + 1]; len + 1];  // dp2[i][j] 把前i个字符分成j段半回文串需要的最少修改次数
    dp2[0][0] = 0;
    // dp2[i][j] = dp2[m][j-1] + dp1[m][i]
    for i in 1..=len {
        for j in 1..=k {
            for m in 0..i - 1 {
                dp2[i][j] = dp2[i][j].min(dp2[m][j - 1] + dp1[m][i - 1]);
            }
        }
    }

    dp2[len][k]
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("baacbbbaba"), 1), 2);
        assert_eq!(func(String::from("edaswf"), 1), 2);
        assert_eq!(func(String::from("acba"), 2), 2);
        assert_eq!(func(String::from("abcac"), 2), 1);
        assert_eq!(func(String::from("abcdef"), 2), 2);
        assert_eq!(func(String::from("aabbaa"), 3), 0);
    }
    test(minimum_changes);
}
