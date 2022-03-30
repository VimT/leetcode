//! 适龄的朋友

pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
    // b > 0.5 * a + 7 && b <= a && (b <= 100 || a >= 100)
    let mut cnt = vec![0; 121];
    for &i in &ages {
        cnt[i as usize] += 1;
    }
    let mut presum = vec![0; 121];
    for i in 1..121 {
        presum[i] = presum[i - 1] + cnt[i];
    }
    let mut ans = 0;
    for i in 0..=120 {
        let left = i / 2 + 8;
        let right = if i >= 100 { i } else { 100.min(i) };
        if left > right { continue; }
        ans += cnt[i] * (presum[right] - presum[left - 1]) - if i >= left && i <= right { cnt[i] } else { 0 };
    }
    ans as i32
}

fn main() {
    assert_eq!(num_friend_requests(vec![20, 112, 32, 29, 101, 73, 8, 95, 73, 50]), 14);
    assert_eq!(num_friend_requests(vec![16, 16]), 2);
    assert_eq!(num_friend_requests(vec![16, 17, 18]), 2);
    assert_eq!(num_friend_requests(vec![20, 30, 100, 110, 120]), 3);
}
