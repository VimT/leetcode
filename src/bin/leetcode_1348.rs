//! 推文计数

use std::collections::{BTreeMap, HashMap};

struct TweetCounts {
    m: HashMap<String, BTreeMap<i32, i32>>,
}

impl TweetCounts {
    fn new() -> Self {
        Self { m: HashMap::new() }
    }

    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        *self.m.entry(tweet_name).or_default().entry(time).or_default() += 1;
    }

    fn get_tweet_counts_per_frequency(&self, freq: String, tweet_name: String, start_time: i32, end_time: i32) -> Vec<i32> {
        let freq = match freq.as_str() {
            "minute" => 60,
            "hour" => 3600,
            "day" => 86400,
            _ => unreachable!()
        };
        let len = (end_time - start_time + freq) / freq;
        let mut result = vec![0; len as usize];
        let tm = match self.m.get(&tweet_name) {
            Some(v) => v,
            None => { return result; }
        };
        for (&t, &cnt) in tm.range(start_time..=end_time) {
            result[((t - start_time) / freq) as usize] += cnt;
        }
        result
    }
}

fn main() {
    let mut tc = TweetCounts::new();
    tc.record_tweet(String::from("tweet3"), 0);
    tc.record_tweet(String::from("tweet3"), 60);
    tc.record_tweet(String::from("tweet3"), 10);                             // "tweet3" 发布推文的时间分别是 0, 10 和 60 。
    assert_eq!(tc.get_tweet_counts_per_frequency(String::from("minute"), String::from("tweet3"), 0, 59), [2]); // 返回 [2]。统计频率是每分钟（60 秒），因此只有一个有效时间间隔 [0,60> - > 2 条推文。
    assert_eq!(tc.get_tweet_counts_per_frequency(String::from("minute"), String::from("tweet3"), 0, 60), [2, 1]); // 返回 [2,1]。统计频率是每分钟（60 秒），因此有两个有效时间间隔 1) [0,60> - > 2 条推文，和 2) [60,61> - > 1 条推文。
    tc.record_tweet(String::from("tweet3"), 120);                            // "tweet3" 发布推文的时间分别是 0, 10, 60 和 120 。
    assert_eq!(tc.get_tweet_counts_per_frequency(String::from("hour"), String::from("tweet3"), 0, 210), [4]);  // 返回 [4]。统计频率是每小时（3600 秒），因此只有一个有效时间间隔 [0,211> - > 4 条推文。
}
