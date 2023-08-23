use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            match map.get(&(target - nums[i])) {
                Some(v) => return vec![*v],
                None => todo!(),
            }
        }

        return vec![];
    }
}
