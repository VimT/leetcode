//! 工作计划的最低难度

pub fn min_difficulty_inner(job_difficulty: Vec<i32>, d: i32) -> i32 {
    fn inner(job: &[i32], count: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        if cache[job.len()][count] > 0 {
            return cache[job.len()][count];
        }
        if count == 1 {
            return *job.iter().max().unwrap();
        }
        let mut result = i32::MAX;
        let mut curmax = 0;
        for i in 0..=job.len() - count {
            curmax = curmax.max(job[i]);
            result = result.min(inner(&job[i + 1..], count - 1, cache) + curmax);
        }
        cache[job.len()][count] = result;
        result
    }
    if d as usize > job_difficulty.len() {
        return -1;
    }
    let mut cache = vec![vec![-1; d as usize + 1]; job_difficulty.len() + 1];
    inner(&job_difficulty, d as usize, &mut cache)
}

pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
    let len = job_difficulty.len();
    let d = d as usize;
    if d > len { return -1; }
    // pre_len split to pre_count
    let mut dp = vec![0; len + 1];

    // dp[3][5] = dp[2][4] + job[4..5]
    //          .max dp[2][3] + job[3..5]
    //          .max dp[2][2] + job[2..5].max()
    let mut curmax = 0;
    for i in 1..=len {
        curmax = curmax.max(job_difficulty[i - 1]);
        dp[i] = curmax;
    }
    for count in 2..=d {
        let mut new_dp = vec![0; len + 1];
        for jobs in count..=len {
            let mut tmp = i32::MAX;
            let mut curmax = 0;
            for i in (count - 1..jobs).rev() {
                curmax = curmax.max(job_difficulty[i]);
                tmp = tmp.min(dp[i] + curmax);
            }
            new_dp[jobs] = tmp;
        }
        dp = new_dp;
    }

    dp[len]
}

fn main() {
    assert_eq!(min_difficulty(vec![6, 5, 4, 3, 2, 1], 2), 7);
    assert_eq!(min_difficulty(vec![143, 446, 351, 170, 117, 963, 785, 76, 139, 772, 452, 743, 23, 767, 564, 872, 922, 532, 957, 945, 203, 615, 843, 909, 458, 320, 290, 235, 174, 814, 414, 669, 422, 769, 781, 721, 523, 94, 100, 464, 484, 562, 941], 5), 1839);
    assert_eq!(min_difficulty(vec![186, 398, 479, 206, 885, 423, 805, 112, 925, 656, 16, 932, 740, 292, 671, 360], 4), 1803);
    assert_eq!(min_difficulty(vec![9, 9, 9], 4), -1);
    assert_eq!(min_difficulty(vec![1, 1, 1], 3), 3);
    assert_eq!(min_difficulty(vec![7, 1, 7, 1, 7, 1], 3), 15);
    assert_eq!(min_difficulty(vec![11, 111, 22, 222, 33, 333, 44, 444], 6), 843);
}
