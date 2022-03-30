//! 汇总区间

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut ans = vec![];
    let mut start = None;
    let mut current = 0;
    let len = nums.len();
    for i in 0..=len {
        let num = if i == len { 0 } else { nums[i] };
        match start {
            Some(v) => {
                if i == len || num != current + 1 {
                    if current != v {
                        ans.push(format!("{}->{}", v, current));
                    } else {
                        ans.push(format!("{}", v));
                    }
                    start = Some(num);
                }
                current = num;
            }
            None => {
                start = Some(num);
                current = num;
            }
        }
    }
    ans
}


fn main() {
    let a = svec!["abc"];
    assert_eq!(summary_ranges(vec![0, 1, 2, 4, 5, 7]), vec!["0->2", "4->5", "7"]);
    assert_eq!(summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]), vec!["0", "2->4", "6", "8->9"]);
}
