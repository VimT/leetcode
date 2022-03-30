//! 基于陈述统计最多好人数

pub fn maximum_good(statements: Vec<Vec<i32>>) -> i32 {
    let len = statements.len();
    let mut result = 0;
    for good in 0..1usize << len {
        let mut ok = true;
        'out: for i in 0..len {
            if good & (1 << i) > 0 {
                for j in 0..len {
                    match statements[i][j] {
                        0 => if good & (1 << j) > 0 {
                            ok = false;
                            break 'out;
                        },
                        1 => if good & (1 << j) == 0 {
                            ok = false;
                            break 'out;
                        },
                        _ => {}
                    }
                }
            }
        }
        if ok {
            result = result.max(good.count_ones())
        }
    }
    result as i32
}

fn main() {
    assert_eq!(maximum_good(vec![vec![2, 1, 2], vec![1, 2, 2], vec![2, 0, 2]]), 2);
    assert_eq!(maximum_good(vec![vec![2, 0], vec![0, 2]]), 1);
}
