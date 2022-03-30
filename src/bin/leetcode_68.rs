//! 文本左右对齐
pub fn full_justify(mut words: Vec<String>, max_width: i32) -> Vec<String> {
    fn justify_line(mut words: Vec<String>, max_width: usize) -> String {
        let word_len: usize = words.iter().map(|x| x.len()).sum();
        let space_len = max_width - word_len;
        let mut ans = vec![];
        let mut space_num = words.len();
        if space_num > 1 { space_num -= 1; }
        let each_space_len = space_len / space_num;
        let mut more = space_len % space_num;
        ans.push(words.remove(0));
        if words.is_empty() {
            ans.push(" ".repeat(each_space_len));
        }
        for i in words {
            let mut space = each_space_len;
            if more > 0 {
                space += 1;
                more -= 1;
            }
            ans.push(" ".repeat(space));
            ans.push(i);
        }
        ans.into_iter().collect()
    }
    let mut ans = vec![];
    let mut current_line = vec![];
    let mut current_len = 0;
    words.push("".to_string());
    let max_width = max_width as usize;
    for word in words {
        if current_len + word.len() + current_line.len() - 1 >= max_width {
            ans.push(justify_line(current_line, max_width));
            current_line = vec![];
            current_len = 0;
        }
        current_len += word.len();
        current_line.push(word);
    }
    if current_len > 0 {
        ans.push(format!("{}{}", current_line.join(" "), " ".repeat(max_width - current_len - (current_line.len() - 1))));
    }
    ans
}

fn main() {
    for i in full_justify("this is an example of text justification.".split(" ").map(|x| x.to_string()).collect(), 16) {
        println!("{}", i);
    }
    for i in full_justify("i".split(" ").map(|x| x.to_string()).collect(), 16) {
        println!("{}", i);
    }
    for i in full_justify(["What", "must", "be", "acknowledgment", "shall", "be"].iter().map(|x| x.to_string()).collect(), 16) {
        println!("{}", i);
    }
    for i in full_justify(["Science", "is", "what", "we", "understand", "well", "enough", "to", "explain", "to", "a", "computer.", "Art", "is", "everything", "else", "we", "do"].iter().map(|x| x.to_string()).collect(), 20) {
        println!("{}", i);
    }
}

