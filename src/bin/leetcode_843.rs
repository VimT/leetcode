//! 猜猜这个单词

use std::cell::RefCell;

use leetcode::svec;

pub struct Master {
    secret: String,
    cnt: RefCell<i32>,
}

impl Master {
    fn new(s: String) -> Self {
        Master { secret: s, cnt: RefCell::new(0) }
    }

    fn guess(&self, word: String) -> i32 {
        *self.cnt.borrow_mut() += 1;
        let s = self.secret.as_bytes();
        let w = word.as_bytes();
        let mut result = 0;
        for i in 0..s.len().min(w.len()) {
            if s[i] == w[i] {
                result += 1;
            }
        }
        result
    }
}

/// 极小化极大算法
pub fn find_secret_word(words: Vec<String>, master: &Master) {
    let len = words.len();
    let mut h = vec![vec![0_u8; len]; len];
    for i in 0..len {
        for j in 0..len {
            let mut mc = 0;
            let s1 = words[i].as_bytes();
            let s2 = words[j].as_bytes();
            for k in 0..6 {
                if s1[k] == s2[k] {
                    mc += 1;
                }
            }
            h[i][j] = mc;
        }
    }
    let mut possible: Vec<usize> = (0..len).collect();
    let mut path = vec![];
    fn solve(h: &Vec<Vec<u8>>, possible: &Vec<usize>, path: &Vec<usize>) -> usize {
        if possible.len() <= 2 { return possible[0]; }
        let mut result = usize::MAX;
        let mut result_group = possible.len();
        for &guess in possible {
            if !path.contains(&guess) {
                let mut group = vec![0; 7];
                for &j in possible {
                    if j != guess {
                        group[h[guess][j] as usize] += 1;
                    }
                }
                let max_len = *group.iter().max().unwrap();
                // 每次去猜桶内计数最大值最小的那个单词,就能尽可能多地, 保险地排除单词.
                // 尽量减少possible（猜测范围）的大小
                if max_len < result_group {
                    result_group = max_len;
                    result = guess;
                }
            }
        }
        result
    }
    while !possible.is_empty() {
        let guess = solve(&h, &possible, &path);
        let matches = master.guess(words[guess].clone()) as u8;
        if matches == 6 { return; }
        let mut new_possible = vec![];
        for j in possible {
            // 当前guess和target match=m,候选只能是和guess match == m的那些单词
            if h[guess][j] == matches {
                new_possible.push(j);
            }
        }
        possible = new_possible;
        path.push(guess);
    }
}

fn main() {
    let master = Master::new(String::from("acckzz"));
    find_secret_word(svec!["ccbazz", "eiowzz", "abcczz", "acckzz"], &master);
    println!("{}", *master.cnt.borrow());
    let master = Master::new(String::from("hamada"));
    find_secret_word(svec!["hamada", "khaled"], &master);
    println!("{}", *master.cnt.borrow());
}
