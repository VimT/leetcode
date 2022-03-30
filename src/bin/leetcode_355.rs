//! [355]设计推特.rs

use std::collections::{HashMap, HashSet, LinkedList};

struct Twitter {
    follow_map: HashMap<i32, HashSet<i32>>,
    post_map: HashMap<i32, LinkedList<(i32, i32)>>,
    count: i32,
}


impl Twitter {
    fn new() -> Self {
        Twitter {
            follow_map: HashMap::new(),
            post_map: HashMap::new(),
            count: 0,
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.count += 1;
        self.post_map.entry(user_id).or_insert(LinkedList::new()).push_front((self.count, tweet_id));
        if let Some(post) = self.post_map.get_mut(&user_id) {
            if post.len() > 10 {
                post.pop_back();
            }
        }
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let followees = self.follow_map.get(&user_id).map(|x| x.iter().cloned().collect::<Vec<i32>>()).unwrap_or(vec![]);
        let mut result = Vec::with_capacity(11);
        if let Some(posts) = self.post_map.get(&user_id) {
            for post in posts {
                result.push(*post);
            }
        }
        for followee in followees {
            if followee == user_id { continue; }
            if let Some(posts) = self.post_map.get(&followee) {
                let mut i = 0;
                for post in posts {
                    while i < result.len() && post.0 < result[i].0 {
                        i += 1;
                    }
                    if i > 10 {
                        break;
                    }
                    result.insert(i, *post);
                    i += 1;
                }
            }
        }
        result[..10.min(result.len())].iter().map(|(_, x)| *x).collect()
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follow_map.entry(follower_id).or_insert(HashSet::new()).insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follow_map.entry(follower_id).or_insert(HashSet::new()).remove(&followee_id);
    }
}

fn main() {
    let mut t = Twitter::new();
    t.post_tweet(1, 5); // 用户 1 发送了一条新推文 (用户 id = 1, 推文 id = 5)
    println!("{:?}", t.get_news_feed(1));  // 用户 1 的获取推文应当返回一个列表，其中包含一个 id 为 5 的推文
    t.follow(1, 2);    // 用户 1 关注了用户 2
    t.post_tweet(2, 6); // 用户 2 发送了一个新推文 (推文 id = 6)
    println!("{:?}", t.get_news_feed(1));  // 用户 1 的获取推文应当返回一个列表，其中包含两个推文，id 分别为 -> [6, 5] 。推文 id 6 应当在推文 id 5 之前，因为它是在 5 之后发送的
    t.unfollow(1, 2);  // 用户 1 取消关注了用户 2
    println!("{:?}", t.get_news_feed(1));  // 用户 1 获取推文应当返回一个列表，其中包含一个 id 为 5 的推文。因为用户 1 已经不再关注用户 2
    for i in (0..9).rev() {
        t.post_tweet(3, i);
    }
    t.follow(1, 3);
    t.post_tweet(1, 11);
    println!("{:?}", t.get_news_feed(1));
}
