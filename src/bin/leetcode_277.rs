//! 搜寻名人

struct Solution {
    adj: Vec<Vec<i32>>,
}

impl Solution {
    fn knows(&self, a: i32, b: i32) -> bool {
        return self.adj[a as usize][b as usize] == 1;
    }

    pub fn find_celebrity(&self, n: i32) -> i32 {
        let mut result = 0;
        for i in 1..n {
            // 如果认识，则result不是名人
            if self.knows(result, i) {
                result = i;
            }
        }
        for i in 0..n {
            if result == i { continue; }
            // 认识别人或者别人不认识他
            if self.knows(result, i) || !self.knows(i, result) {
                return -1;
            }
        }
        result
    }
}


fn main() {
    let help = |adj: Vec<Vec<i32>>| {
        let n = adj.len();
        let s = Solution { adj };
        return s.find_celebrity(n as i32);
    };
    assert_eq!(help(vec![vec![1, 1, 0], vec![0, 1, 0], vec![1, 1, 1]]), 1);
    assert_eq!(help(vec![vec![1, 0, 1], vec![1, 1, 0], vec![0, 1, 1]]), -1);
}
