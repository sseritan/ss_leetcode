use std::collections::HashMap;

struct Solution;
impl Solution {
    // For each side, need to find entries that sum up to a side
    // If I knew it was exactly two entries, could do the complement trick from 0001
    // I just need to find ONE subsum, so can try and adapt https://www.geeksforgeeks.org/subset-sum-problem-dp-25/
    fn findsubsum(nums: &[i32], target: i32) -> Option<Vec<usize>> {
        // Key is subsum, value is list of indices to build key
        let mut subset: HashMap<i32, Vec<usize>> = HashMap::new();
        subset.insert(0, vec![]); // Base case

        for i in 0..nums.len() {
            let val = nums[i];

            for subsum in 1..target+1 {
                let subsum = subsum as i32;

                if subset.contains_key(&subsum) {
                    //println!("Index: {} (val: {}) subsum: {} subset: {:?}... skipping", i, val, subsum, subset);
                    continue;
                }

                let remainder = subsum - val;
                if let Some(ssindex) = subset.get(&remainder) {
                    if !ssindex.contains(&i) { // Avoid repeats
                        let mut index = ssindex.clone();
                        index.push(i);

                        if subsum == target {
                            //println!("Index: {} (val: {}) subsum: {} subset: {:?}... returning", i, val, subsum, subset);
                            return Some(index)
                        } else {
                            subset.insert(subsum, index);    
                        }
                    }
                } // end if let Some(ssindex)

                //println!("Index: {} (val: {}) subsum: {} subset: {:?}", i, val, subsum, subset);
            } // end for subsum
        } // end for i

        subset.remove(&target)
    } // end fn findsubsum

    pub fn makesquare(mut nums: Vec<i32>) -> bool {
        let length: i32 = (nums.iter().sum::<i32>()) / 4;

        // Edge cases: No sticks or any stick too long
        if nums.iter().any(|&num| num > length) || length == 0 {
            return false
        }

        for side in 0..4 {
            match Solution::findsubsum(&nums[..], length) {
                None => {
                    return false
                },
                Some(indices) => {
                    println!("Found {:?} for side {}", indices, side);
                    for i in indices.iter().rev() {
                        nums.remove(*i);
                    }                    
                }
            }
        }

        true
    }
}

fn main() {
    let input = vec![1,1,2,2,2];
    println!("Testing {:?}...", input);
    let output = Solution::makesquare(input);
    println!("Expected true, got {}", output);

    let input = vec![3,3,3,3,4];
    println!("Testing {:?}...", input);
    let output = Solution::makesquare(input);
    println!("Expected false, got {}", output);

    let input = vec![];
    println!("Testing {:?}...", input);
    let output = Solution::makesquare(input);
    println!("Expected false, got {}", output);

    let input = vec![5,5,5,5,4,4,4,4,3,3,3,3];
    println!("Testing {:?}...", input);
    let output = Solution::makesquare(input);
    println!("Expected true, got {}", output);

}
