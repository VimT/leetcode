//! 切分数组

static mut MIN_PRIME: [usize; 1000010] = [0; 1000010];
static mut PRIME: [usize; 100010] = [0; 100010];
static mut G: [usize; 1000010] = [0; 1000010];

pub fn split_array(nums: Vec<i32>) -> i32 {
    unsafe {
        let len = nums.len();
        let mut m = 2;
        for i in 0..len {
            m = m.max(nums[i] as usize)
        }
        for i in 2..=m {
            if MIN_PRIME[i] == 0 {
                PRIME[0] += 1;
                PRIME[PRIME[0]] = i;
                MIN_PRIME[i] = i;
            }
            for j in 1..=PRIME[0] {
                if i > m / PRIME[j] { break; }
                MIN_PRIME[(i * PRIME[j])] = PRIME[j];
                if i % PRIME[j] == 0 { break; }
            }
        }
        for j in 1..=PRIME[0] {
            G[PRIME[j]] = len;
        }
        let nums: Vec<usize> = nums.into_iter().map(|x| x as usize).collect();
        let mut x = nums[0];
        while x > 1 {
            G[MIN_PRIME[x]] = 0;
            x /= MIN_PRIME[x];
        }
        let mut ans = 1;
        for i in 0..len {
            ans = i + 1;
            x = nums[i];
            while x > 1 {
                ans = ans.min(G[MIN_PRIME[x]] + 1);
                x /= MIN_PRIME[x];
            }
            if i == len - 1 { break; }
            x = nums[i + 1];
            while x > 1 {
                G[MIN_PRIME[x]] = ans.min(G[MIN_PRIME[x]]);
                x /= MIN_PRIME[x];
            }
        }

        ans as i32
    }
}


fn main() {
    assert_eq!(split_array(vec![2, 3, 3, 2, 3, 3]), 2);
    assert_eq!(split_array(vec![2, 3, 5, 7]), 4);
}
