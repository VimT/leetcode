//! 第K个语法符号

pub fn kth_grammar(_: i32, k: i32) -> i32 {
    ((k - 1).count_ones() & 1) as i32
}

fn main() {
    assert_eq!(kth_grammar(1, 1), 0);
    assert_eq!(kth_grammar(2, 1), 0);
    assert_eq!(kth_grammar(2, 2), 1);
}
