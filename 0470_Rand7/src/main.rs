/** 
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */
use rand::Rng;
fn rand7() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1, 8);
}

struct Solution;
impl Solution {
    pub fn rand10() -> i32 {
        let mut val = 50;
        while val > 40 {
            let a = rand7();
            assert!(a < 8);
            let b = rand7();
            assert!(b < 8);
            val = a + (b-1)*7;
            //println!("a: {} b: {} val: {} ", a, b, val);
        }
        1 + (val - 1) % 10 // 1-indexing
    }
}

fn test(n: i32) {
    println!("Pulling {} samples", n);
    for _ in 0..n {
        println!("{} ", Solution::rand10());
    }
}

fn test_dist() {
    let mut freq = vec![0; 10];
    let samples = 10000;

    println!("Testing distribution with {:?} samples", samples);

    for _ in 0..samples {
        let bin = Solution::rand10() - 1;
        freq[bin as usize] += 1;
    }

    for f in freq {
        print!("{} ", (f as f32)/(samples as f32));
    }
    println!();
}

fn main() {
    test(1);
    test(3);

    test_dist();
}
