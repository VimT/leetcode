//! 在线选举

struct TopVotedCandidate {
    times: Vec<i32>,
    result: Vec<i32>,
}


impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut m = vec![0; 5001];
        let mut cur_max = 0;
        let mut cur = 0;
        let mut result = Vec::with_capacity(persons.len());
        for i in persons {
            m[i as usize] += 1;
            if m[i as usize] >= cur_max {
                cur_max = m[i as usize];
                cur = i;
            }
            result.push(cur);
        }

        TopVotedCandidate {
            times,
            result,
        }
    }

    fn q(&self, t: i32) -> i32 {
        let idx = self.times.binary_search(&t).unwrap_or_else(|x| x - 1);
        return self.result[idx];
    }
}

fn main() {
    let t = TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
    assert_eq!(t.q(3), 0);
    assert_eq!(t.q(12), 1);
    assert_eq!(t.q(25), 1);
    assert_eq!(t.q(15), 0);
    assert_eq!(t.q(24), 0);
    assert_eq!(t.q(8), 1);
}
