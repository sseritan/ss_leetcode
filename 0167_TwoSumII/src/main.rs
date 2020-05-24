use std::collections::HashMap;

struct Solution ();

// The exact same solution as the original TwoSum should work, just the 1-indexing
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
                    indices.push(*val + 1);
                    indices.push(i as i32 + 1);
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
