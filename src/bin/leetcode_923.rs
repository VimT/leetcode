//! 三数之和的多种可能


pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut m = vec![0; 101];
    let mut min = arr[0];
    let mut max = arr[0];
    for num in arr {
        m[num as usize] += 1;
        min = min.min(num);
        max = max.max(num);
    }
    let mut result = 0;
    for i in min..max {
        for j in i + 1..=max {
            let k = target - i - j;
            if k > j && k <= max {
                result = (result + m[i as usize] * m[j as usize] * m[k as usize]) % MOD;
            }
        }
    }
    for i in min..max {
        if m[i as usize] > 1 {
            let k = target - 2 * i;
            if k > i && k <= max {
                result = (result + m[i as usize] * (m[i as usize] - 1) / 2 * m[k as usize]) % MOD;
            }
        }
    }
    for i in min..max {
        let mut j = target - i;
        if j & 1 == 0 {
            j >>= 1;
            if j > i && j <= max && m[j as usize] > 1 {
                result = (result + m[i as usize] * (m[j as usize] * (m[j as usize] - 1) / 2)) % MOD;
            }
        }
    }
    if target % 3 == 0 {
        let cnt = m[(target / 3) as usize];
        if cnt >= 3 {
            result = (result + cnt * (cnt - 1) * (cnt - 2) / 6) % MOD;
        }
    }
    result as i32
}

fn main() {
    assert_eq!(three_sum_multi(vec![92, 4, 59, 23, 100, 16, 7, 15, 3, 78, 98, 17, 77, 33, 83, 15, 87, 35, 54, 72, 58, 14, 87, 47, 58, 31, 72, 58, 87, 22, 25, 54, 27, 53, 13, 54, 61, 12, 96, 24, 35, 43, 94, 1, 88, 76, 89, 89, 41, 56, 61, 65, 60, 91, 89, 79, 86, 52, 27, 2, 97, 46, 50, 46, 87, 93, 71, 87, 95, 78, 65, 10, 35, 51, 34, 66, 61, 7, 49, 38, 10, 1, 88, 37, 50, 84, 35, 20, 7, 83, 51, 85, 11, 12, 89, 93, 54, 23, 36, 95, 100, 19, 82, 67, 96, 77, 53, 56, 51, 16, 54, 7, 30, 68, 78, 13, 38, 52, 91, 44, 54, 17, 21, 44, 4, 10, 85, 19, 11, 88, 73, 36, 47, 53, 3, 21, 41, 24, 60, 53, 88, 35, 48, 89, 35, 3, 43, 85, 45, 67, 56, 78, 44, 49, 29, 35, 68, 96, 29, 21, 51, 17, 52, 99, 3, 48, 65, 51, 22, 38, 77, 81, 30, 64, 99, 35, 88, 72, 73, 29, 29, 2],
                               105), 20);
    assert_eq!(three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8), 20);
    assert_eq!(three_sum_multi(vec![1, 1, 2, 2, 2, 2], 5), 12);
}
