//! 等价多米诺骨牌对的数量

pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut count = [[0; 10]; 10];
    for i in dominoes.iter() {
        if i[0] > i[1] {
            count[i[1] as usize][i[0] as usize] += 1;
        } else {
            count[i[0] as usize][i[1] as usize] += 1;
        }
    }
    let mut ans = 0;
    for i in 1..10 {
        for j in i..10 {
            if count[i][j] > 1 {
                ans += count[i][j] * (count[i][j] - 1) / 2;
            }
        }
    }
    ans
}


fn main() {
    assert_eq!(num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]), 1);
    assert_eq!(num_equiv_domino_pairs(vec![vec![1, 2], vec![1, 2], vec![1, 1], vec![1, 2], vec![2, 2]]), 3);
}
