//! 得分最高的单词集合

use leetcode::svec;

pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut letter_num = 0;
    let mut cnt = [0; 26];
    for &ch in &letters {
        letter_num |= 1 << ch as u8 - b'a';
        cnt[(ch as u8 - b'a') as usize] += 1;
    }
    let words: Vec<[u8; 26]> = words.into_iter().filter_map(|s| {
        let mut num = 0;
        let mut cnt = [0; 26];
        for &ch in s.as_bytes() {
            num |= 1 << ch - b'a';
            cnt[(ch - b'a') as usize] += 1;
        }
        if num & letter_num == num {
            Some(cnt)
        } else { None }
    }).collect();

    fn dfs(words: &Vec<[u8; 26]>, ori_cnt: &[u8], score: &Vec<i32>, i: usize, cnt: [u8; 26], result: &mut i32) {
        if i == words.len() {
            (*result) = (*result).max((0..26).map(|x| {
                score[x] * (ori_cnt[x] - cnt[x]) as i32
            }).sum());
            return;
        }
        let mut ok = true;
        let wcnt = &words[i];
        let mut next_cnt = cnt.clone();
        for i in 0..26 {
            if cnt[i] < wcnt[i] {
                ok = false;
                break;
            }
            next_cnt[i] = cnt[i] - wcnt[i];
        }
        if ok {
            dfs(words, ori_cnt, score, i + 1, next_cnt, result);
        }
        dfs(words, ori_cnt, score, i + 1, cnt, result);
    }
    let mut result = 0;
    dfs(&words, &cnt, &score, 0, cnt.clone(), &mut result);
    result
}

fn main() {
    fn test(func: fn(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32) {
        assert_eq!(func(svec!["dog","cat","dad","good"], vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'], vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 23);
        assert_eq!(func(svec!["xxxz","ax","bx","cx"], vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'], vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]), 27);
        assert_eq!(func(svec!["leetcode"], vec!['l', 'e', 't', 'c', 'o', 'd'], vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]), 0);
    }
    test(max_score_words);
}
