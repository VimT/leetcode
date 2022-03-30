//! 负二进制数相加

pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let n1 = arr1.len();
    let n2 = arr2.len();
    let n = n1.max(n2) + 4;
    let mut result = vec![0; n];
    // 和arr1和arr2 倒序的计算，低位在前面
    for i in (0..n1).rev() {
        result[n1 - 1 - i] += arr1[i];
    }
    for i in (0..n2).rev() {
        result[n2 - 1 - i] += arr2[i];
    }
    // 从低位开始计算
    for i in 0..n - 2 {
        // 进位
        let carry = result[i] >> 1;
        result[i] &= 1;
        result[i + 1] += carry;
        result[i + 2] += carry;
    }
    // 观察最高位连续零需要移除
    let mut k = n - 3;
    // 这里结束是0，来避免0,0->空的情况
    while k > 0 && result[k] == 0 {
        k -= 1;
    }
    unsafe { result.set_len(k + 1); }
    result.reverse();
    result
}

fn main() {
    assert_eq!(add_negabinary(vec![1, 1, 1, 1, 1], vec![1, 0, 1]), vec![1, 0, 0, 0, 0]);
    assert_eq!(add_negabinary(vec![0], vec![0]), vec![0]);
    assert_eq!(add_negabinary(vec![0], vec![1]), vec![1]);
}
