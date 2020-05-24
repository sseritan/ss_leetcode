use std::collections::HashMap;

struct Solution ();

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices : Vec<i32> = Vec::new();
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let remainder = target - num;
            
            match map.get(&remainder) {
                None => {
                    // No match yet, save remainder for later referencing
                    map.insert(num, i as i32);
                },
                Some(val) => {
                    // Remainder was previously seen, return pair
                    indices.push(*val);
                    indices.push(i as i32);
                    return indices
                }
            }
        }

        indices
    }
}

fn main() {
    let nums = vec![1, 2, 6, 7, 11, 15];
    let target = 9;

    let indices = Solution::two_sum(nums, target);
    println!("Found indices: {:?}", indices);
}
