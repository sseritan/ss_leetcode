use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_lonely_pixel(picture: Vec<Vec<char>>) -> i32 {
        let mut seen_rows: HashMap<usize, bool> = HashMap::new();
        let mut seen_cols: HashMap<usize, bool> = HashMap::new();
        let mut pixels: Vec<(usize,usize)> = vec![];
        
        for (i, row) in picture.iter().enumerate() {
            for (j, letter) in row.iter().enumerate() {
                if *letter == 'B' {
                    match seen_rows.get_mut(&i) {
                        None => {
                            seen_rows.insert(i, false);
                        },
                        Some(val) => {
                            *val = true;
                        }
                    }
                    match seen_cols.get_mut(&j) {
                        None => {
                            seen_cols.insert(j, false);
                        },
                        Some(val) => {
                            *val = true;
                        }
                    }

                    pixels.push((i,j));
                }
            }
        }

        let mut lonely_count = 0;
        for (i,j) in pixels {
            if let Some(rval) = seen_rows.get(&i) {
                if let Some(cval) = seen_cols.get(&j) {
                    if !rval && !cval {
                        println!("Lonely pixel at ({}, {})", i, j);
                        lonely_count += 1;
                    }
                }
            }
        }

        lonely_count
    }
}

fn main() {
    //let input = vec![vec!['B','W','W','W','W','B','W','B','B','W'],
    //                 vec!['B','B','B','W','W','W','B','W','B','W'],
    //                 vec!['B','B','B','B','W','W','W','W','W','W'],
    //                 vec!['B','W','W','B','W','B','B','W','W','W'],
    //                 vec!['W','W','B','W','B','B','B','W','B','B'],
    //                 vec!['W','B','B','W','W','W','B','W','W','W'],
    //                 vec!['B','W','W','B','B','B','W','W','W','W'],
    //                 vec!['W','W','W','B','B','B','B','W','W','W'],
    //                 vec!['W','W','B','B','W','W','W','W','B','W'],
    //                 vec!['W','W','W','B','B','B','W','W','W','B']];

    let input = vec![vec!['B','W','W'],
                     vec!['W','B','W'],
                     vec!['W','W','B']];

    println!("Input array: {:?}", input);
    println!("Found {} lonely pixels", Solution::find_lonely_pixel(input));
}
