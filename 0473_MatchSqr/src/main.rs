struct Solution;
impl Solution {
    fn dfs(nums: &Vec<i32>, sides: &mut Vec<i32>, length: i32, index: usize) -> bool {
        // Base case: If last element, 3 sides must have been completed
        if index == nums.len() {
            println!("Base case returning");
            return sides[0] == length && sides[1] == length && sides[2] == length;
        }

        // Try adding current stick to each side in turn
        // If recursive call is true, solution is good so keep unwinding
        // Otherwise, keep trying next side and return false when all sides fail
        let val = nums[index];
        println!("Index: {:?} (val {}) Current sides: {:?}", index, val, sides);
        for i in 0..4 {
            if sides[i] + val <= length {
                println!("Trying index {} (val {}) to side {}", index, val, i);
                sides[i] += val;
                if Solution::dfs(nums, sides, length, index+1) {
                    println!("Unwrapping index {}", index);
                    return true
                }
                println!("Unwrapped back to index {}", index);
                sides[i] -= val;
            } // end if sides[i]
        } // end for i

        return false
    }

    pub fn makesquare(mut nums: Vec<i32>) -> bool {
        let length: i32 = (nums.iter().sum::<i32>()) / 4;

        // Edge cases: No sticks or any stick too long
        if nums.iter().any(|&num| num > length) || length == 0 {
            return false
        }

        // O(NlogN) sort to descending (cheap for 15 elements)
        nums.sort_unstable_by(|a, b| b.cmp(a));

        // Depth-first search
        let mut sides: Vec<i32> = vec![0; 4];
        sides[0] = nums[0]; // Can assign first element to one side to skip outer recursion loop
        Solution::dfs(&nums, &mut sides, length, 1)
    }
}

fn main() {
    //let input = vec![1,1,2,2,2];
    //println!("Testing {:?}...", input);
    //let output = Solution::makesquare(input);
    //println!("Expected true, got {}", output);

    //let input = vec![3,3,3,3,4];
    //println!("Testing {:?}...", input);
    //let output = Solution::makesquare(input);
    //println!("Expected false, got {}", output);

    //let input = vec![];
    //println!("Testing {:?}...", input);
    //let output = Solution::makesquare(input);
    //println!("Expected false, got {}", output);

    let input = vec![5,5,5,5,4,4,4,4,3,3,3,3];
    println!("Testing {:?}...", input);
    let output = Solution::makesquare(input);
    println!("Expected true, got {}", output);

}
