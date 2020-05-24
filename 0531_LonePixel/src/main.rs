use std::collections::HashMap;

struct Solution;
impl Solution {
    // Should be O(N) runtime and memory
    pub fn find_lonely_pixel(picture: Vec<Vec<char>>) -> i32 {
        // Key as 0-indexed 2D index
        // Value: False is seen, True is seen and has neighbors
        let mut table: HashMap<(i32,i32), bool> = HashMap::new();
        
        for (i, row) in picture.iter().enumerate() {
            for (j, letter) in row.iter().enumerate() {
                let mut has_neighbor = false;
                
                if *letter == 'B' {
                    // Check if previous neighbors are in table already
                    if let Some(neighbor_val) = table.get_mut(&(i as i32 -1, j as i32)) {
                        *neighbor_val = true;
                        has_neighbor = true;
                    }
                    if let Some(neighbor_val) = table.get_mut(&(i as i32, j as i32 -1)) {
                        *neighbor_val = true;
                        has_neighbor = true;
                    }

                    // Store new pixel
                    table.insert((i as i32, j as i32), has_neighbor);
                }
            }
        }

        let mut lonely_count = 0;
        // Any False in table is a lonely pixel
        for (key, val) in table.iter() {
            if !val {
                println!("Lonely at {:?}", key);
                lonely_count += 1;
            }
        }

        lonely_count
    }
}

fn main() {
    let input = vec![vec!['B','W','W','W','W','B','W','B','B','W'],
                     vec!['B','B','B','W','W','W','B','W','B','W'],
                     vec!['B','B','B','B','W','W','W','B','W','W'],
                     vec!['B','W','W','B','W','B','B','W','W','W'],
                     vec!['W','W','B','W','B','B','B','W','B','B'],
                     vec!['W','B','B','W','W','W','B','W','W','W'],
                     vec!['B','W','W','B','B','B','W','W','W','W'],
                     vec!['W','W','W','B','B','B','B','W','W','W'],
                     vec!['W','W','B','B','W','W','W','W','B','W'],
                     vec!['W','W','W','B','B','B','W','W','W','B']];

    println!("Input array: {:?}", input);
    println!("Found {} lonely pixels", Solution::find_lonely_pixel(input));
}
