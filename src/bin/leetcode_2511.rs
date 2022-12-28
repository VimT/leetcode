//! 最多可以摧毁的敌人城堡数目

pub fn capture_forts(forts: Vec<i32>) -> i32 {
    let len = forts.len();
    let mut result = 0;
    for i in 0..len {
        if forts[i] == 1 {
            // 向左
            let mut j = i;
            while j > 0 && forts[j - 1] == 0 {
                j -= 1;
            }
            if j > 0 && forts[j - 1] == -1 {
                result = result.max(i - j);
            }
            // 向右
            j = i + 1;
            while j < len && forts[j] == 0 {
                j += 1;
            }
            if j < len && forts[j] == -1 {
                result = result.max(j - i - 1);
            }
        }
    }
    result as i32
}

fn main() {
    assert_eq!(capture_forts(vec![1, 0, 0, -1, 0, 0, 0, 0, 1]), 4);
    assert_eq!(capture_forts(vec![0, 0, 1, -1]), 0);
}
