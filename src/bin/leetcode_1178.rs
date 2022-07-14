//! 猜字谜

use leetcode::svec;

// 1140ms
pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    let len = words.len();
    let mut word_chars = vec![0; len];
    for i in 0..len {
        let mut repl = 0;
        for &char in words[i].as_bytes() {
            repl = repl | (1 << (char - b'a'));
        }
        word_chars[i] = repl;
    }
    let mut ans = vec![];

    for puzzle in puzzles {
        let puzzle = puzzle.as_bytes();
        let mut has_char = 0;
        for &char in puzzle {
            has_char = has_char | (1 << (char - b'a'));
        }
        let mut count = 0;
        for i in 0..len {
            if word_chars[i] & (1 << (puzzle[0] - b'a')) == 0 {
                continue;
            }
            if word_chars[i] & has_char != word_chars[i] {
                continue;
            }
            count += 1;
        }
        ans.push(count);
    }

    ans
}

/// 状态压缩+子集
pub fn find_num_of_valid_words_bin_compact(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    // 出现相同字母的word出现的次数
    let mut frequency = std::collections::HashMap::new();
    for word in &words {
        let mut mask: i32 = 0;
        for ch in word.as_bytes() {
            mask |= 1 << ch - b'a';
        }
        if mask.count_ones() <= 7 {
            *frequency.entry(mask).or_insert(0) += 1;
        }
    }
    let mut ans = Vec::with_capacity(puzzles.len());
    for puzzle in puzzles {
        let puzzle = puzzle.as_bytes();
        let mut total = 0;
        // method 1, 36ms
        // for choose in 0..1<<6  {
        //     let mut mask = 0;
        //     for i in 0..6 {
        //         if choose & (1 << i) > 0 {
        //             mask |= 1 << puzzle[i + 1] - b'a';
        //         }
        //     }
        //     mask |= 1 << puzzle[0] - b'a';
        //     if let Some(&count) = frequency.get(&mask) {
        //         total += count;
        //     }
        // }

        // method 2, 32ms
        let mut mask = 0;
        for i in 1..7 {
            mask |= 1 << puzzle[i] - b'a';
        }
        let mut subset = mask;
        // 看字母一模一样的，有多少个word
        while subset > 0 {
            let s = subset | (1 << puzzle[0] - b'a');
            if let Some(&count) = frequency.get(&s) {
                total += count;
            }
            // remove last 1
            subset = (subset - 1) & mask;
        }
        // only one char
        if let Some(&count) = frequency.get(&(1 << puzzle[0] - b'a')) {
            total += count;
        }
        ans.push(total);
    }

    ans
}

#[derive(Default, Debug)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    frequency: i32,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, s: &[u8]) {
        let mut node = self;
        for &i in s {
            let idx = i as usize;
            node = node.children[idx].get_or_insert_with(Box::<Trie>::default);
        }
        node.frequency += 1;
    }
}

// 76ms
pub fn find_num_of_valid_words_trie(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    let mut root = Some(Box::new(Trie::new()));
    fn find(puzzle: &[u8], required: u8, cur: &Option<Box<Trie>>, pos: usize) -> i32 {
        if cur.is_none() { return 0; }
        if pos == 7 {
            return cur.as_ref().unwrap().frequency;
        }
        let mut ret = 0;
        let idx = puzzle[pos] - b'a';
        if cur.as_ref().unwrap().children[idx as usize].is_some() {
            ret += find(puzzle, required, &cur.as_ref().unwrap().children[idx as usize], pos + 1);
        }
        if puzzle[pos] != required {
            ret += find(puzzle, required, cur, pos + 1);
        }
        return ret;
    }
    for word in words {
        let chars = word.into_bytes();
        let mut count = [false; 26];
        for char in chars {
            count[(char - b'a') as usize] = true;
        }
        let mut char = Vec::with_capacity(26);
        for i in 0..26 {
            if count[i] {
                char.push(i as u8);
            }
        }
        if char.len() <= 7 {
            root.as_mut().unwrap().insert(&char);
        }
    }
    let mut ans = Vec::with_capacity(puzzles.len());
    for puzzle in puzzles {
        let mut puzzle = puzzle.into_bytes();
        let required = puzzle[0];
        puzzle.sort_unstable();
        ans.push(find(&puzzle, required, &root, 0));
    }
    ans
}


fn main() {
    fn test(func: fn(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32>) {
        assert_eq!(func(svec!["aaaa","asas","able","ability","actt","actor","access"], svec!["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"]), vec![1, 1, 3, 2, 4, 0]);
        assert_eq!(func(svec!["apple","pleas","please"], svec!["aelwxyz","aelpxyz","aelpsxy","saelpxy","xaelpsy"]), vec![0, 1, 3, 2, 0]);
    }
    test(find_num_of_valid_words);
    test(find_num_of_valid_words_bin_compact);
    test(find_num_of_valid_words_trie);
}
