//! 按递增顺序显示卡牌

use std::collections::VecDeque;

pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
    let mut q = VecDeque::with_capacity(deck.len());
    deck.sort_unstable();
    deck.reverse();
    q.push_back(deck[0]);
    for i in 1..deck.len() {
        let last = q.pop_back().unwrap();
        q.push_front(last);
        q.push_front(deck[i]);
    }
    q.into()
}

fn main() {
    assert_eq!(deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]), vec![2, 13, 3, 11, 5, 17, 7]);
    assert_eq!(deck_revealed_increasing(vec![1, 1000]), vec![1, 1000]);
}
