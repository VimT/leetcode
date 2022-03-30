//! 分发糖果

pub fn candy(ratings: Vec<i32>) -> i32 {
    let len = ratings.len();
    let mut help = vec![1; len];
    for i in 1..len {
        if ratings[i] > ratings[i - 1] {
            help[i] = help[i - 1] + 1;
        }
    }
    for i in (0..len - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            help[i] = help[i].max(help[i + 1] + 1);
        }
    }

    help.into_iter().sum()
}


fn main() {
    assert_eq!(candy(vec![1, 0, 2]), 5);
    assert_eq!(candy(vec![1, 2, 2]), 4);
}
