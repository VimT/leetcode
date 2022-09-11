//! 设计食物评分系统

use std::collections::{BTreeSet, HashMap};
use leetcode::svec;

struct FoodRatings {
    food_cuisines: HashMap<String, String>,
    food_rating: HashMap<String, i32>,
    cuisine_rating: HashMap<String, BTreeSet<(i32, String)>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut fc = HashMap::new();
        let mut cr: HashMap<String, BTreeSet<(i32, String)>> = HashMap::new();
        let mut fr = HashMap::new();
        for ((food, cuisine), rating) in foods.into_iter().zip(cuisines.into_iter()).zip(ratings.into_iter()) {
            fc.insert(food.clone(), cuisine.clone());
            fr.insert(food.clone(), rating);
            cr.entry(cuisine).or_default().insert((-rating, food));
        }
        FoodRatings { food_cuisines: fc, food_rating: fr, cuisine_rating: cr }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let ori = self.food_rating.get(&food).cloned().unwrap();
        let cuisine = self.food_cuisines.get(&food).unwrap();
        let rating_set = self.cuisine_rating.get_mut(cuisine).unwrap();
        rating_set.remove(&(-ori, food.clone()));
        rating_set.insert((-new_rating, food.clone()));
        self.food_rating.insert(food, new_rating);
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let set = self.cuisine_rating.get(&cuisine).unwrap();
        set.range(..).next().unwrap().1.clone()
    }
}

fn main() {
    let mut fd = FoodRatings::new(svec!["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"], svec!["korean", "japanese", "japanese", "greek", "japanese", "korean"], vec![9, 12, 8, 15, 14, 7]);
    assert_eq!(fd.highest_rated(String::from("korean")), "kimchi"); // 返回 "kimchi"
    // "kimchi" 是分数最高的韩式料理，评分为 9 。
    assert_eq!(fd.highest_rated(String::from("japanese")), "ramen"); // 返回 "ramen"
    // "ramen" 是分数最高的日式料理，评分为 14 。
    fd.change_rating(String::from("sushi"), 16); // "sushi" 现在评分变更为 16 。
    assert_eq!(fd.highest_rated(String::from("japanese")), "sushi"); // 返回 "sushi"
    // "sushi" 是分数最高的日式料理，评分为 16 。
    fd.change_rating(String::from("ramen"), 16); // "ramen" 现在评分变更为 16 。
    assert_eq!(fd.highest_rated(String::from("japanese")), "ramen"); // 返回 "ramen"
    // "sushi" 和 "ramen" 的评分都是 16 。
    // 但是，"ramen" 的字典序比 "sushi" 更小。
}
