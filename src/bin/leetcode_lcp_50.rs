//! 2022春季编程大赛：宝石补给

pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
    for op in operations {
        let give = gem[op[0] as usize] / 2;
        gem[op[0] as usize] -= give;
        gem[op[1] as usize] += give;
    }
    *gem.iter().max().unwrap() - *gem.iter().min().unwrap()
}

fn main() {
    assert_eq!(give_gem(vec![3, 1, 2], vec![vec![0, 2], vec![2, 1], vec![2, 0]]), 2);
    assert_eq!(give_gem(vec![100, 0, 50, 100], vec![vec![0, 2], vec![0, 1], vec![3, 0], vec![3, 0]]), 75);
    assert_eq!(give_gem(vec![0, 0, 0, 0], vec![vec![1, 2], vec![3, 1], vec![1, 2]]), 0);
}
