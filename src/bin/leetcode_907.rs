//! 子数组的最小值之和

/// 单调栈，计算每个位置的左边最小值和右边最小值（其中有一个包含重复元素有一个不包含）。然后每个位置左边数量×右边数量×数本身
pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let len = arr.len();
    let mut left_min = vec![-1; len];
    let mut right_min = vec![len as i32; len];
    let mut s = vec![];
    for i in 0..len {
        while !s.is_empty() && arr[i] < arr[*s.last().unwrap()] {
            let p = s.pop().unwrap();
            right_min[p] = i as i32;
        }
        s.push(i);
    }
    s.clear();
    for i in (0..len).rev() {
        while !s.is_empty() && arr[i] <= arr[*s.last().unwrap()] {
            let p = s.pop().unwrap();
            left_min[p] = i as i32;
        }
        s.push(i);
    }
    let mut result = 0;
    for i in 0..len {
        // !!!
        result = (result + (i as i32 - left_min[i]) as i64 * (right_min[i] - i as i32) as i64 * arr[i] as i64) % MOD;
    }
    result as i32
}

/// 最小值栈
/// [1,7,5,2,4,3,9] [i..=6] 各子数组的最小值是 [1,2,2,2,3,3,9]
/// 维护单调栈为 [(val=1, count=1), (val=2, count=3), (val=3, count=2), (val=9, count=1)]
/// nb..
pub fn sum_subarray_mins2(arr: Vec<i32>) -> i32 {
    let mut s: Vec<(i32, i32)> = vec![];
    let mut ans = 0;
    let mut stack_sum = 0;
    for end in 0..arr.len() {
        let mut count = 1;
        while !s.is_empty() && s.last().unwrap().0 > arr[end] {
            let (v, c) = s.pop().unwrap();
            // 对于前面比当前数大的，减掉，cnt加上这些数量
            stack_sum -= v * c;
            count += c;
        }
        s.push((arr[end], count));
        // stack_num: 以end结尾的所有子数组的最小值和
        stack_sum += arr[end] * count;
        ans += stack_sum;
        ans = ans % (1000_000_000 + 7);
    }
    ans
}

fn main() {
    assert_eq!(sum_subarray_mins(vec![71, 55, 82, 55]), 593);
    assert_eq!(sum_subarray_mins(vec![3, 1, 2, 4]), 17);
    assert_eq!(sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
}
