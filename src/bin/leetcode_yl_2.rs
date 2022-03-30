//! 银联-02. 优惠活动系统

use std::collections::HashMap;

struct Activity {
    id: i32,
    price_limit: i32,
    discount: i32,
    number: i32,
    user_limit: i32,
}

struct DiscountSystem {
    activities: Vec<Activity>,
    user_activity: HashMap<(i32, i32), i32>,
}


impl DiscountSystem {
    fn new() -> Self {
        Self { activities: Vec::new(), user_activity: HashMap::new() }
    }

    fn add_activity(&mut self, act_id: i32, price_limit: i32, discount: i32, number: i32, user_limit: i32) {
        self.activities.push(Activity { id: act_id, price_limit, discount, number, user_limit });
    }

    fn remove_activity(&mut self, act_id: i32) {
        if let Some(idx) = self.activities.iter().position(|x| x.id == act_id) {
            self.activities.remove(idx);
        }
    }

    fn consume(&mut self, user_id: i32, cost: i32) -> i32 {
        let act = self.activities.iter().enumerate().filter(|(_, x)| {
            cost >= x.price_limit && x.number > 0 && *self.user_activity.get(&(user_id, x.id)).unwrap_or(&0) < x.user_limit
        }).max_by_key(|(_, x)| (x.discount, -x.id));
        if let Some((idx, _)) = act {
            self.activities[idx].number -= 1;
            *self.user_activity.entry((user_id, self.activities[idx].id)).or_insert(0i32) += 1;
            cost - self.activities[idx].discount
        } else {
            cost
        }
    }
}

fn main() {
    let mut obj = DiscountSystem::new(); // 初始化系统
    obj.add_activity(1, 10, 6, 3, 2); // 创建编号 1 的优惠活动，单笔消费原价不小于 10 时， 可享受 6 的减免，优惠活动共有 3 个名额，每个用户最多参与该活动 2 次
    obj.add_activity(2, 15, 8, 8, 2); // 创建编号 2 的优惠活动，单笔消费原价不小于 15 时， 可享受 8 的减免，优惠活动共有 8 个名额，每个用户最多参与该活动 2 次
    obj.consume(101, 13); // 用户 101 消费了 13，仅满足优惠活动 1 条件，返回实际支付 13 - 6 = 7 用户 101 参加 1 次活动 1，活动 1 剩余 2 个名额
    obj.consume(101, 8); // 用户 101 消费了 8，不满足任何活动，返回支付原价 8
    obj.remove_activity(2); // 结束编号为 2 的活动
    obj.consume(101, 17); // 用户 101 消费了 17，满足优惠活动 1 条件，返回实际支付 17 - 6 = 11 用户 101 参加 2 次活动 1，活动 1 剩余 1 个名额
    obj.consume(101, 11); // 用户 101 消费了 11，满足优惠活动 1 条件，但已达到参加次数限制，返回支付原价 11
    obj.consume(102, 16); // 用户 102 消费了 16，满足优惠活动 1 条件，返回实际支付 16 - 6 = 10 用户 102 参加 1 次活动 1，活动 1 无剩余名额
    obj.consume(102, 21); // 用户 102 消费了 21，满足优惠活动 1 条件，但活动 1 已无剩余名额，返回支付原价 21
}
