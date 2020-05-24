struct Solution;
impl Solution {
    pub fn makesquare(mut nums: Vec<i32>) -> bool {
        let length: i32 = (nums.iter().sum::<i32>()) / 4;

        // Edge cases: No sticks or any stick too long
        if nums.iter().any(|&num| num > length) || length == 0 {
            return false
        }

        // O(NlogN) sort in descending
        nums.sort_unstable_by(|a, b| b.cmp(a));

        for _side in 0..4 {
            println!("Searching for side {}", _side);
            let mut indices = vec![];
            let mut remainder = length;

            for (i,num) in nums.iter().enumerate() {
                if *num <= remainder {
                    remainder -= *num;
                    indices.insert(0, i); // Prepend to avoid index issues in remove
                }
                if remainder == 0 {
                    break;
                }
            }

            if remainder != 0 {
                // Could not make a side of right length
                return false
            }

            println!("Found side: {:?}", indices);
            for i in indices {
                nums.remove(i);
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
