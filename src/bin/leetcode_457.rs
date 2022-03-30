//! 环形数组是否存在循环

pub fn circular_array_loop(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut tried = vec![false; len];
    for i in 0..len {
        if !tried[i] {
            let mut vis = vec![false; len];
            let mut cur = i as i32;
            let positive = nums[i] > 0;
            vis[i] = true;
            loop {
                if nums[cur as usize].abs() % len as i32 == 0 { break; }
                cur = (cur + nums[cur as usize]) % len as i32;
                if cur < 0 { cur += len as i32; }
                if (positive && nums[cur as usize] < 0) || (!positive && nums[cur as usize] > 0) || tried[cur as usize] {
                    break;
                }
                if vis[cur as usize] {
                    return true;
                }
                vis[cur as usize] = true;
                cur = cur;
            }
            for j in 0..len {
                if vis[j] {
                    tried[j] = true;
                }
            }
        }
    }
    false
}

/// 快慢指针 O(n) and O(1)
pub fn circular_array_loop_best(mut nums: Vec<i32>) -> bool {
    let len = nums.len() as i32;
    let np = nums.as_ptr();
    let next = |i: i32| -> i32{
        unsafe { (i + *np.add(i as usize) % len + len) % len }
    };
    for i in 0..len {
        if nums[i as usize] == 0 {
            continue;
        }
        let mut slow = i;
        let mut fast = next(i);
        while nums[slow as usize] * nums[fast as usize] > 0 && nums[slow as usize] * nums[next(fast) as usize] > 0 {
            if slow == fast {
                if slow != next(slow) {
                    return true;
                } else {
                    break;
                }
            }
            slow = next(slow);
            fast = next(next(fast));
        }
        let mut add = i;
        while nums[add as usize] * nums[next(add) as usize] > 0 {
            let nxt = next(add);
            nums[add as usize] = 0;
            add = nxt;
        }
    }
    false
}

fn main() {
    assert_eq!(circular_array_loop_best(vec![-2, -3, -9]), false);
    assert_eq!(circular_array_loop_best(vec![-1, -2, -3, -4, -5]), false);
    assert_eq!(circular_array_loop_best(vec![-1, 2]), false);
    assert_eq!(circular_array_loop_best(vec![2, -1, 1, 2, 2]), true);
    assert_eq!(circular_array_loop_best(vec![-2, 1, -1, -2, -2]), false);
}
