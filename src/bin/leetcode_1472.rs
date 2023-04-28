//! 设计浏览器历史记录

struct BrowserHistory {
    history: Vec<String>,
    i: usize,
}


impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self { history: vec![homepage], i: 1 }
    }

    fn visit(&mut self, url: String) {
        self.history.truncate(self.i);
        self.history.push(url);
        self.i += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        self.i = (self.i as i32 - steps).max(1) as usize;
        self.history[self.i - 1].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.i = (self.i + steps as usize).min(self.history.len());
        self.history[self.i - 1].clone()
    }
}

fn main() {
    let mut bh = BrowserHistory::new(String::from("leetcode.com"));
    bh.visit(String::from("google.com"));       // 你原本在浏览 "leetcode.com" 。访问 "google.com"
    bh.visit(String::from("facebook.com"));     // 你原本在浏览 "google.com" 。访问 "facebook.com"
    bh.visit(String::from("youtube.com"));      // 你原本在浏览 "facebook.com" 。访问 "youtube.com"
    assert_eq!(bh.back(1), "facebook.com");                   // 你原本在浏览 "youtube.com" ，后退到 "facebook.com" 并返回 "facebook.com"
    assert_eq!(bh.back(1), "google.com");                   // 你原本在浏览 "facebook.com" ，后退到 "google.com" 并返回 "google.com"
    assert_eq!(bh.forward(1), "facebook.com");                // 你原本在浏览 "google.com" ，前进到 "facebook.com" 并返回 "facebook.com"
    bh.visit(String::from("linkedin.com"));     // 你原本在浏览 "facebook.com" 。 访问 "linkedin.com"
    assert_eq!(bh.forward(2), "linkedin.com");                // 你原本在浏览 "linkedin.com" ，你无法前进任何步数。
    assert_eq!(bh.back(2), "google.com");                   // 你原本在浏览 "linkedin.com" ，后退两步依次先到 "facebook.com" ，然后到 "google.com" ，并返回 "google.com"
    assert_eq!(bh.back(7), "leetcode.com");                   // 你原本在浏览 "google.com"， 你只能后退一步到 "leetcode.com" ，并返回 "leetcode.com"
}
