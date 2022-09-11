//! 最好的扑克手牌

pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
    let suit = suits[0];
    if suits.iter().all(|x| *x == suit) {
        return String::from("Flush");
    }
    let mut cnt = [0; 14];
    for rank in ranks {
        cnt[rank as usize] += 1;
    }
    let max_cnt = *cnt.iter().max().unwrap();
    return if max_cnt >= 3 {
        String::from("Three of a Kind")
    } else if max_cnt >= 2 {
        String::from("Pair")
    } else {
        String::from("High Card")
    };
}

fn main() {
    fn test(func: fn(ranks: Vec<i32>, suits: Vec<char>) -> String) {
        assert_eq!(func(vec![13, 2, 3, 1, 9], vec!['a', 'a', 'a', 'a', 'a']), String::from("Flush"));
        assert_eq!(func(vec![4, 4, 2, 4, 4], vec!['d', 'a', 'a', 'b', 'c']), String::from("Three of a Kind"));
        assert_eq!(func(vec![10, 10, 2, 12, 9], vec!['a', 'b', 'c', 'a', 'd']), String::from("Pair"));
    }
    test(best_hand);
}
