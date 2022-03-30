//! 令牌放置

pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
    if tokens.is_empty() { return 0; }
    tokens.sort_unstable();
    let len = tokens.len();
    let mut start = 0;
    let mut end = len - 1;
    let mut cur_score = 0;
    while start <= end {
        if tokens[start] > power {
            if cur_score == 0 {
                break;
            }
            power += tokens[end] - tokens[start];
            start += 1;
            end -= 1;
        } else {
            power -= tokens[start];
            start += 1;
            cur_score += 1;
        }
    }
    cur_score
}

fn main() {
    assert_eq!(bag_of_tokens_score(vec![100, 200], 150), 1);
    assert_eq!(bag_of_tokens_score(vec![100, 200, 300, 400], 200), 2);
    assert_eq!(bag_of_tokens_score(vec![100], 50), 0);
}
