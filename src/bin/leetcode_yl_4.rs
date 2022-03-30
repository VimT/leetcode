//! 银联-04. 合作开发

use std::collections::HashMap;

pub fn coop_develop(mut skills: Vec<Vec<i32>>) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    // 按照长度排序
    skills.sort_unstable_by_key(|x| x.len());
    let mut cnt = HashMap::new();
    let len = skills.len();
    let mut result = 0;
    for i in 0..len {
        // 找前i个人里，有多少个是自己的子集
        let m = skills[i].len();
        let mut sub_set_cnt = 0;
        for j in 1..1 << m {
            let mut num = 0;
            for k in 0..m {
                if j >> k & 1 == 1 {
                    num = num * 1001 + skills[i][k] as i64;
                }
            }
            sub_set_cnt += *cnt.get(&num).unwrap_or(&0);
        }
        let mut num = 0;
        for j in 0..m {
            num = num * 1001 + skills[i][j] as i64;
        }
        result = (result + i as i32 - sub_set_cnt) % MOD;
        *cnt.entry(num).or_insert(0i32) += 1;
    }
    result
}

fn main() {
    assert_eq!(coop_develop(vec![vec![1, 2, 3], vec![3], vec![2, 4]]), 2);
    assert_eq!(coop_develop(vec![vec![3], vec![6]]), 1);
}
