//! 24 点游戏

use Operate::*;

enum Operate {
    PLUS,
    MINUS,
    MULTIPLY,
    DIV,
    RMINUS,
    RDIV,
}

static OPS: [Operate; 6] = [PLUS, MINUS, MULTIPLY, DIV, RMINUS, RDIV];


pub fn judge_point24(nums: Vec<i32>) -> bool {
    fn inner(nums: &Vec<f64>) -> bool {
        let len = nums.len();
        for i in 0..len {
            for j in i + 1..len {
                let mut new_nums = vec![];
                for k in 0..len {
                    if k != i && k != j { new_nums.push(nums[k]); }
                }
                new_nums.push(0_f64);
                for op in &OPS {
                    let num = match op {
                        PLUS => nums[i] + nums[j],
                        MINUS => nums[i] - nums[j],
                        MULTIPLY => nums[i] * nums[j],
                        DIV => nums[i] / nums[j],
                        RMINUS => nums[j] - nums[i],
                        RDIV => nums[j] / nums[i]
                    };
                    if len == 2 {
                        if (num - 24f64).abs() < 1e-6_f64 { return true; }
                    } else {
                        *new_nums.last_mut().unwrap() = num;
                        if inner(&new_nums) { return true; }
                    }
                }
            }
        }
        false
    }
    let nums = nums.into_iter().map(|x| x as f64).collect::<Vec<f64>>();
    inner(&nums)
}


fn main() {
    assert_eq!(judge_point24(vec![3, 3, 8, 8]), true);
    assert_eq!(judge_point24(vec![1, 4, 6, 1]), true);
    assert_eq!(judge_point24(vec![4, 1, 8, 7]), true);
    assert_eq!(judge_point24(vec![1, 2, 1, 2]), false);
}
