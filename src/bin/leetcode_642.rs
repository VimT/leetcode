//! 设计搜索自动补全系统

// 在trie的每个节点，存储前缀的所有句子。搜索时直接遍历。52ms
mod solution1 {
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    use std::ptr::{null, null_mut};
    use std::rc::Rc;

    use leetcode::svec;

    #[derive(Default)]
    struct Trie {
        children: [Option<Box<Trie>>; 27],
        sentences: Vec<Rc<String>>,
    }

    impl Trie {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn insert(&mut self, sentence: Rc<String>) {
            let mut node = self;
            for &ch in sentence.as_bytes() {
                let idx = if ch == b' ' { 26 } else { (ch - b'a') as usize };
                node = node.children[idx].get_or_insert_with(Box::<Trie>::default);
                node.sentences.push(sentence.clone());
            }
        }
    }

    struct AutocompleteSystem {
        map: HashMap<Rc<String>, i32>,
        trie: Trie,
        cur: Vec<char>,
        cur_node: *const Trie,
        is_first: bool,
    }


    impl AutocompleteSystem {
        fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
            let map: HashMap<Rc<String>, i32> = sentences.into_iter().map(|x| Rc::new(x)).zip(times).collect();
            let mut trie = Trie::new();
            for sentence in map.keys() {
                trie.insert(sentence.clone())
            }
            Self { map, trie, cur: vec![], cur_node: null(), is_first: true }
        }

        fn input(&mut self, c: char) -> Vec<String> {
            if self.is_first {
                // 不能在new的时候赋值，因为自引用，return的时候会move，不能确定引用位置
                self.cur_node = &self.trie as *const _;
                self.is_first = false;
            }
            if c == '#' {
                let sentence: Rc<String> = Rc::new(self.cur.iter().collect());
                match self.map.entry(sentence.clone()) {
                    Entry::Occupied(mut v) => { *v.get_mut() += 1; }
                    Entry::Vacant(v) => {
                        self.trie.insert(sentence.clone());
                        v.insert(1);
                    }
                }
                self.cur_node = &self.trie as *const _;
                self.cur.clear();
                return vec![];
            }
            self.cur.push(c);
            let ch = c as u8;
            let idx = if ch == b' ' { 26 } else { (ch - b'a') as usize };
            unsafe {
                return if self.cur_node.is_null() {
                    vec![]
                } else {
                    self.cur_node = match &(*self.cur_node).children[idx] {
                        None => { null_mut() }
                        Some(nxt) => {
                            &**nxt as *const Trie
                        }
                    };
                    if self.cur_node.is_null() {
                        return vec![];
                    }
                    let mut a = (0, Rc::new(String::new()));
                    let mut b = (0, Rc::new(String::new()));
                    let mut c = (0, Rc::new(String::new()));
                    for sentence in &(*self.cur_node).sentences {
                        let time = self.map[sentence];
                        let d = (-time, sentence.clone());
                        if d < a {
                            c = b;
                            b = a;
                            a = d;
                        } else if d < b {
                            c = b;
                            b = d;
                        } else if d < c {
                            c = d;
                        }
                    }
                    let mut result = Vec::with_capacity(3);
                    if a.0 < 0 {
                        result.push(a.1.to_string());
                    }
                    if b.0 < 0 {
                        result.push(b.1.to_string());
                    }
                    if c.0 < 0 {
                        result.push(c.1.to_string());
                    }
                    result
                };
            }
        }
    }

    pub fn test() {
        let mut obj = AutocompleteSystem::new(svec!["i love you", "island", "iroman", "i love leetcode"], vec![5, 3, 2, 2]);
        assert_eq!(obj.input('i'), svec!["i love you", "island", "i love leetcode"]);
        assert_eq!(obj.input(' '), svec!["i love you", "i love leetcode"]);
        assert_eq!(obj.input('a'), Vec::<String>::new());
        assert_eq!(obj.input('#'), Vec::<String>::new());
        assert_eq!(obj.input('i'), svec!["i love you", "island", "i love leetcode"]);
        assert_eq!(obj.input(' '), svec!["i love you", "i love leetcode", "i a"]);
        assert_eq!(obj.input('a'), svec!["i a"]);
        assert_eq!(obj.input('#'), Vec::<String>::new());
        assert_eq!(obj.input('i'), svec!["i love you", "island", "i a"]);
        assert_eq!(obj.input(' '), svec!["i love you", "i a", "i love leetcode"]);
        assert_eq!(obj.input('a'), svec!["i a"]);
        assert_eq!(obj.input('#'), Vec::<String>::new());
    }
}

// 在 trie 中句子末尾，存储 times，搜索时遍历所有子树。64ms
mod solution2 {
    use std::collections::VecDeque;
    use std::ptr::null_mut;

    use leetcode::svec;

    #[derive(Default)]
    struct Trie {
        children: [Option<Box<Trie>>; 27],
        times: i32,
        sentence: String,
    }

    impl Trie {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn insert(&mut self, sentence: String, times: i32) {
            let mut node = self;
            for &ch in sentence.as_bytes() {
                let idx = if ch == b' ' { 26 } else { (ch - b'a') as usize };
                node = node.children[idx].get_or_insert_with(Box::<Trie>::default);
            }
            node.sentence = sentence;
            node.times = times;
        }
    }

    struct AutocompleteSystem {
        trie: Trie,
        cur_node: *mut Trie,
        is_first: bool,
        cur: Vec<char>,
    }


    impl AutocompleteSystem {
        fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
            let mut trie = Trie::new();
            for (sentence, times) in sentences.into_iter().zip(times) {
                trie.insert(sentence, times);
            }
            Self { trie, cur_node: null_mut(), is_first: true, cur: vec![] }
        }

        fn input(&mut self, c: char) -> Vec<String> {
            unsafe {
                if self.is_first {
                    // 不能在new的时候赋值，因为自引用，return的时候会move，不能确定引用位置
                    self.cur_node = &mut self.trie as *mut _;
                    self.is_first = false;
                }
                if c == '#' {
                    (*self.cur_node).times += 1;
                    (*self.cur_node).sentence = self.cur.iter().collect();
                    self.cur_node = &mut self.trie as *mut _;
                    self.cur.clear();
                    return vec![];
                }
                self.cur.push(c);
                let ch = c as u8;
                let idx = if ch == b' ' { 26 } else { (ch - b'a') as usize };
                if self.cur_node.is_null() {
                    return vec![];
                }
                let nxt = (*self.cur_node).children[idx].get_or_insert_with(Box::<Trie>::default);
                self.cur_node = &mut **nxt as *mut Trie;

                let mut a = (0, "");
                let mut b = (0, "");
                let mut c = (0, "");

                let mut q = VecDeque::new();
                q.push_back(&*nxt);
                while !q.is_empty() {
                    let node = q.pop_front().unwrap();
                    if node.times > 0 {
                        let d = (-node.times, node.sentence.as_str());
                        if d < a {
                            c = b;
                            b = a;
                            a = d;
                        } else if d < b {
                            c = b;
                            b = d;
                        } else if d < c {
                            c = d;
                        }
                    }
                    for i in 0..27 {
                        if let Some(nxt) = &node.children[i] {
                            q.push_back(nxt);
                        }
                    }
                }

                let mut result = Vec::with_capacity(3);
                if a.0 < 0 {
                    result.push(a.1.to_string());
                }
                if b.0 < 0 {
                    result.push(b.1.to_string());
                }
                if c.0 < 0 {
                    result.push(c.1.to_string());
                }
                result
            }
        }
    }

    pub fn test() {
        let mut obj = AutocompleteSystem::new(svec!["i love you", "island", "iroman", "i love leetcode"], vec![5, 3, 2, 2]);
        assert_eq!(obj.input('i'), svec!["i love you", "island", "i love leetcode"]);
        assert_eq!(obj.input(' '), svec!["i love you", "i love leetcode"]);
        assert_eq!(obj.input('a'), Vec::<String>::new());
        assert_eq!(obj.input('#'), Vec::<String>::new());
        assert_eq!(obj.input('i'), svec!["i love you", "island", "i love leetcode"]);
        assert_eq!(obj.input(' '), svec!["i love you", "i love leetcode", "i a"]);
        assert_eq!(obj.input('a'), svec!["i a"]);
        assert_eq!(obj.input('#'), Vec::<String>::new());
        assert_eq!(obj.input('i'), svec!["i love you", "island", "i a"]);
        assert_eq!(obj.input(' '), svec!["i love you", "i a", "i love leetcode"]);
        assert_eq!(obj.input('a'), svec!["i a"]);
        assert_eq!(obj.input('#'), Vec::<String>::new());
    }
}


fn main() {
    solution1::test();
    solution2::test();
}
