//! 单词搜索 II

use leetcode::{svec, unorder};

#[derive(Default, Debug)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, s: &String) {
        let mut node = self;
        for i in s.as_bytes() {
            let idx = (i - b'a') as usize;
            node = node.children[idx].get_or_insert_with(Box::<Trie>::default);
        }
        node.is_word = true;
    }
}

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    fn inner(board: &Vec<Vec<char>>, node: &mut Trie, x: usize, y: usize, current_word: &mut Vec<char>, visited: &mut Vec<Vec<bool>>, ans: &mut Vec<String>) {
        visited[x][y] = true;
        let char = board[x][y];
        let next = &mut node.children[(char as u8 - b'a') as usize];
        if next.is_some() {
            // println!("current access: board[{}][{}]: {}, {:?}", x, y, board[x][y], char);
            let next = next.as_mut().unwrap();
            current_word.push(char);
            if next.is_word {
                let w = current_word.iter().collect::<String>();
                ans.push(w);
                next.is_word = false;
            }
            if x > 0 && !visited[x - 1][y] { inner(board, next, x - 1, y, current_word, visited, ans); }
            if y > 0 && !visited[x][y - 1] { inner(board, next, x, y - 1, current_word, visited, ans); }
            if x + 1 < board.len() && !visited[x + 1][y] { inner(board, next, x + 1, y, current_word, visited, ans); }
            if y + 1 < board[0].len() && !visited[x][y + 1] { inner(board, next, x, y + 1, current_word, visited, ans); }
            current_word.pop();
        }
        visited[x][y] = false;
    }

    let visited = vec![vec![false; board[0].len()]; board.len()];
    let mut ans = vec![];
    let mut trie = Trie::new();
    for i in words {
        trie.insert(&i);
    }

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            inner(&board, &mut trie, i, j, &mut vec![], &mut visited.clone(), &mut ans);
        }
    }
    ans
}


fn main() {
    assert_eq!(unorder(find_words(vec![vec!['o', 'a', 'a', 'n'], vec!['e', 't', 'a', 'e'], vec!['i', 'h', 'k', 'r'], vec!['i', 'f', 'l', 'v']], svec!["oath", "pea", "eat", "rain"])), vec!["eat", "oath"]);
    assert_eq!(unorder(find_words(vec![vec!['a', 'b'], vec!['c', 'd']], svec!["abcb"])).is_empty(), true);
}
