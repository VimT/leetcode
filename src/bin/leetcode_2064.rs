//! 分配给商店的最多商品的最小值

pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    let mut max = 0;
    for &i in &quantities {
        max = max.max(i);
    }
    let mut left = 0;
    let mut right = max;
    while left < right {
        let mid = (left + right) / 2;
        if mid == 0 {
            left = mid + 1;
            continue;
        }
        let mut cnt = 0;
        for &i in &quantities {
            cnt += i / mid;
            if i % mid != 0 {
                cnt += 1;
            }
        }
        if cnt <= n {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    assert_eq!(minimized_maximum(1, vec![1]), 1);
    assert_eq!(minimized_maximum(22, vec![25, 11, 29, 6, 24, 4, 29, 18, 6, 13, 25, 30]), 13);
    assert_eq!(minimized_maximum(6, vec![11, 6]), 3);
    assert_eq!(minimized_maximum(7, vec![15, 10, 10]), 5);
    assert_eq!(minimized_maximum(1, vec![100000]), 100000);
}
