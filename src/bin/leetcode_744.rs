//! 寻找比目标字母大的最小字母


pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    for &ch in &letters {
        if ch > target {
            return ch;
        }
    }
    return letters[0];
}

fn main() {
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'd'), 'f');
}
