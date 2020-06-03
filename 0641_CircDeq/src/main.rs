struct MyCircularDeque {
    vals: Vec<i32>,
    ftail: i32, // Location of tail of front
    rhead: i32, // Location of head of rear
    size: i32,  // Size of vals (convenience as int)
}
impl MyCircularDeque {

    /** Initialize your data structure here. Set the size of the deque to be k. */
    fn new(k: i32) -> Self {
        MyCircularDeque {
            vals: vec![0; k as usize],
            ftail: -1,
            rhead: k,
            size: k,
        }
    }
    
    /** Adds an item at the front of Deque. Return true if the operation is successful. */
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false
        }

        for i in (0..self.ftail+1).rev() {
            let next = (i + 1) as usize;
            self.vals[next] = self.vals[i as usize];
        }

        self.ftail += 1;
        self.vals[0] = value;
        true
    }
    
    /** Adds an item at the rear of Deque. Return true if the operation is successful. */
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false
        }
        
        for i in self.rhead-1..self.size-1 {
            let next = (i + 1) as usize;
            self.vals[i as usize] = self.vals[next];
        }

        self.rhead -= 1;
        self.vals[(self.size - 1) as usize] = value;
        true
    }
    
    /** Deletes an item from the front of Deque. Return true if the operation is successful. */
    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false
        }
        
        if self.ftail >= 0 {
            for i in 0..self.ftail {
                let next = (i as i32 + 1) as usize;
                self.vals[i as usize] = self.vals[next];
            }
            self.ftail -= 1;
        } else {
            // Edge case where nothing in front but something in rear
            self.vals[self.rhead as usize] = 0;
            self.rhead += 1;
        }
        true
    }
    
    /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false
        }

        if self.rhead < self.size {
            for i in (self.rhead..self.size).rev() {
                let prev = (i as i32 - 1) as usize;
                self.vals[i as usize] = self.vals[prev];
            }
            self.rhead += 1;
        } else {
            // Edge case where nothing in rear but something in front
            self.vals[self.ftail as usize] = 0;
            self.ftail -= 1;
        }
        true
    }
    
    /** Get the front item from the deque. */
    fn get_front(&self) -> i32 {
        if self.ftail >= 0 {
            return self.vals[0]
        } else if self.rhead < self.size {
            return self.vals[self.rhead as usize]
        }
        
        -1 // Dummy return
    }
    
    /** Get the last item from the deque. */
    fn get_rear(&self) -> i32 {
        if self.rhead < self.size {
            return self.vals[(self.size - 1) as usize];
        } else if self.ftail >= 0 {
            return self.vals[self.ftail as usize];
        }
        
        -1 // Dummy return
    }
    
    /** Checks whether the circular deque is empty or not. */
    fn is_empty(&self) -> bool {
        self.ftail == -1 && self.rhead == self.size
    }
    
    /** Checks whether the circular deque is full or not. */
    fn is_full(&self) -> bool {
        self.ftail + 1 == self.rhead        
    }
}

use std::fmt;
impl fmt::Display for MyCircularDeque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ftail == -1 {
            write!(f, "^[").expect("Error printing MyCircularDeque");
        } else {
            write!(f, "[").expect("Error printing MyCircularDeque");
        }

        for (i, v) in self.vals.iter().enumerate() {
            let mut prefix = "";
            let mut suffix = "";
            if i as i32 == self.ftail {
                suffix = "^";
            } else if i as i32 == self.rhead {
                prefix = "$";
            }

            if i as i32 == self.size - 1 {
                write!(f, "{}{}{}", prefix, *v, suffix).expect("Error printing MyCircularDeque");
            } else {
                write!(f, "{}{}{}, ", prefix, *v, suffix).expect("Error printing MyCircularDeque");
            }
        }

        if self.rhead == self.size {
            write!(f, "]$")
        } else {
            write!(f, "]")
        }
    }
}

fn test1() {
    let mut cdq = MyCircularDeque::new(3);  // ^[_,_,_]$ (vals with ftail ^ and rhead $)
    assert!(cdq.insert_front(1) == true);   // [1^,_,_]$
    assert!(cdq.insert_last(2) == true);    // [1^,_,$2]
    assert!(cdq.is_empty() == false);       // [1^,_,$2], false
    assert!(cdq.is_full() == false);        // [1^,_,$2], false
    assert!(cdq.insert_last(3) == true);    // [1^,$2,3]
    assert!(cdq.insert_last(4) == false);   // [1^,$2,3], fails
    assert!(cdq.insert_front(4) == false);  // [1^,$2,3], fails
    assert!(cdq.is_full() == true);         // [1^,$2,3], true
    assert!(cdq.get_front() == 1);          // [1^,$2,3]
    assert!(cdq.get_rear() == 3);           // [1^,$2,3]
    assert!(cdq.delete_front() == true);    // ^[_,$2,3]
    assert!(cdq.get_front() == 2);          // ^[_,$2,3] (edge case where get front goes to rear head)
    assert!(cdq.delete_last() == true);     // ^[_,_,$2]
    assert!(cdq.delete_front() == true);    // ^[_,_,_]$ (funny edge case where remove front messes with rear indexing)
    assert!(cdq.delete_front() == false);   // ^[_,_,_]$, false
    assert!(cdq.delete_last() == false);    // ^[_,_,_]$, false
    assert!(cdq.is_empty() == true);        // ^[_,_,_]$, true
    println!("Test 1 passed");
}

fn test2() {
    let mut cdq = MyCircularDeque::new(3);      // ^[_,_,_]$ (vals with ftail ^ and rhead $)
    assert!(cdq.insert_front(9) == true);       // [9^,_,_]$
    assert!(cdq.get_rear() == 9);               // [9^,_,_]$
    assert!(cdq.insert_front(9) == true);       // [9,9^,_]$
    println!("{}", cdq);
    assert!(cdq.get_rear() == 9);               // [9,9^,_]$
    println!("Test 2 passed");
}

enum Output {
    Num(i32),
    Bool(bool),
}

// Verbose version of LeetCode test to track down bug
fn leetcode_test(k: i32, funcs: Vec<&str>, input: Vec<Vec<i32>>, output: Vec<Output>) -> i32 {
    let mut cdq = MyCircularDeque::new(k);

    let mut errors = 0;
    for (f, (i, o)) in funcs.iter().zip(input.iter().zip(output.iter())) {
        match *f {
            "insertFront" => {
                let ret = cdq.insert_front(i[0]);
                match *o {
                    Output::Bool(b) => {
                        if b == ret {
                            println!("Successful insert_front({}): {}", i[0], cdq);
                        } else {
                            println!("FAILED insert_front({}): {}", i[0], cdq);
                            errors += 1;
                        }
                    },
                    _ => println!("Expected bool in Output!"),
                }
            } ,
            "insertLast" => {
                let ret = cdq.insert_last(i[0]);
                match *o {
                    Output::Bool(b) => {
                        if b == ret {
                            println!("Successful insert_last({}): {}", i[0], cdq);
                        } else {
                            println!("FAILED insert_last({}): {}", i[0], cdq);
                            errors += 1;
                        }
                    },
                    _ => println!("Expected bool in Output!"),
                }
            },
            "deleteFront" => {
                let ret = cdq.delete_front();
                match *o {
                    Output::Bool(b) => {
                        if b == ret {
                            println!("Successful delete_front(): {}", cdq);
                        } else {
                            println!("FAILED delete_front(): {}", cdq);
                            errors += 1;
                        }
                    },
                    _ => println!("Expected bool in Output!"),
                }
            },
            "deleteLast" => {
                let ret = cdq.delete_last();
                match *o {
                    Output::Bool(b) => {
                        if b == ret {
                            println!("Successful delete_last(): {}", cdq);
                        } else {
                            println!("FAILED delete_last(): {}", cdq);
                            errors += 1;
                        }
                    },
                    _ => println!("Expected bool in Output!"),
                }
            },
            "getFront" => {
                let ret = cdq.get_front();
                match *o {
                    Output::Num(n) => {
                        if n == ret {
                            println!("Successful get_front(): Got {} from {}", ret, cdq);
                        } else {
                            println!("FAILED get_front(): Expected {}, got {} from {}", n, ret, cdq);
                            errors += 1;
                        }
                    },
                    _ => println!("Expected i32 in Output!"),
                }
            },
            "getRear" => {
                let ret = cdq.get_rear();
                match *o {
                    Output::Num(n) => {
                        if n == ret {
                            println!("Successful get_rear(): Got {} from {}", ret, cdq);
                        } else {
                            println!("FAILED get_rear(): Expected {}, got {} from {}", n, ret, cdq);
                            errors += 1;
                        }
                    },
                    _ => println!("Expected i32 in Output!"),
                }
            },
            "isFull" => {
                let ret = cdq.is_full();
                match *o {
                    Output::Bool(b) => {
                        if b == ret {
                            println!("Successful is_full(): Got {} from {}", ret, cdq);
                        } else {
                            println!("FAILED is_full(): Expected {}, got {} from {}", b, ret, cdq);
                            errors += 1;
                        }
                    },
                    _ => println!("Expected bool in Output!"),
                }
            },
            "isEmpty" => {
                let ret = cdq.is_empty();
                match *o {
                    Output::Bool(b) => {
                        if b == ret {
                            println!("Successful is_empty(): Got {} from {}", ret, cdq);
                        } else {
                            println!("FAILED is_empty(): Expected {}, got {} from {}", b, ret, cdq);
                            errors += 1;
                        }
                    },
                    _ => println!("Expected bool in Output!"),
                }
            },
            _ => panic!("Not implemented: {}", f),
        }
    }

    errors
}

fn test3() {
    let k = 20; // 77 in test, but I think this is enough space for same behaviour and easier to read
    let funcs = vec!["insertFront","getRear","deleteLast","getRear","insertFront","insertFront","insertFront","insertFront","isFull","insertFront",
        "isFull","getRear","deleteLast","getFront","getFront","insertLast","deleteFront","getFront","insertLast","getRear",
        "insertLast","getRear","getFront","getFront","getFront","getRear","getRear","insertFront","getFront","getFront",
        "getFront","getFront","deleteFront","insertFront","getFront","deleteLast","insertLast","insertLast","getRear","getRear",
        "getRear","isEmpty","insertFront","deleteLast","getFront","deleteLast","getRear","getFront","isFull","isFull",
        "deleteFront","getFront","deleteLast","getRear","insertFront","getFront","insertFront","insertFront","getRear","isFull",
        "getFront","getFront","insertFront","insertLast","getRear","getRear","deleteLast","insertFront","getRear","insertLast",
        "getFront","getFront","getFront","getRear","insertFront","isEmpty","getFront","getFront","insertFront","deleteFront",
        "insertFront","deleteLast","getFront","getRear","getFront","insertFront","getFront","deleteFront","insertFront","isEmpty",
        "getRear","getRear","getRear","getRear","deleteFront","getRear","isEmpty","deleteFront","insertFront","insertLast",
        "deleteLast"]; // 101 instructions, test case 22
    let inputs = vec![vec![89],vec![],vec![],vec![],vec![19],vec![23],vec![23],vec![82],vec![],vec![45],vec![],vec![],vec![],vec![],vec![],vec![74],vec![],vec![],vec![98],vec![],
        vec![99],vec![],vec![],vec![],vec![],vec![],vec![],vec![8],vec![],vec![],vec![],vec![],vec![],vec![75],vec![],vec![],vec![35],vec![59],vec![],vec![],
        vec![],vec![],vec![22],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![21],vec![],vec![26],vec![63],vec![],vec![],
        vec![],vec![],vec![87],vec![76],vec![],vec![],vec![],vec![26],vec![],vec![67],vec![],vec![],vec![],vec![],vec![36],vec![],vec![],vec![],vec![72],vec![],
        vec![87],vec![],vec![],vec![],vec![],vec![85],vec![],vec![],vec![91],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![34],vec![44],
        vec![]];
    let outputs = vec![Output::Bool(true),Output::Num(89),Output::Bool(true),Output::Num(-1),Output::Bool(true),Output::Bool(true),Output::Bool(true),Output::Bool(true),Output::Bool(false),Output::Bool(true),
        Output::Bool(false),Output::Num(19),Output::Bool(true),Output::Num(45),Output::Num(45),Output::Bool(true),Output::Bool(true),Output::Num(82),Output::Bool(true),Output::Num(98),
        Output::Bool(true),Output::Num(99),Output::Num(82),Output::Num(82),Output::Num(82),Output::Num(99),Output::Num(99),Output::Bool(true),Output::Num(8),Output::Num(8),
        Output::Num(8),Output::Num(8),Output::Bool(true),Output::Bool(true),Output::Num(75),Output::Bool(true),Output::Bool(true),Output::Bool(true),Output::Num(59),Output::Num(59),
        Output::Num(59),Output::Bool(false),Output::Bool(true),Output::Bool(true),Output::Num(22),Output::Bool(true),Output::Num(98),Output::Num(22),Output::Bool(false),Output::Bool(false),
        Output::Bool(true),Output::Num(75),Output::Bool(true),Output::Num(74),Output::Bool(true),Output::Num(21),Output::Bool(true),Output::Bool(true),Output::Num(74),Output::Bool(false),
        Output::Num(63),Output::Num(63),Output::Bool(true),Output::Bool(true),Output::Num(76),Output::Num(76),Output::Bool(true),Output::Bool(true),Output::Num(74),Output::Bool(true),
        Output::Num(26),Output::Num(26),Output::Num(26),Output::Num(67),Output::Bool(true),Output::Bool(false),Output::Num(36),Output::Num(36),Output::Bool(true),Output::Bool(true),
        Output::Bool(true),Output::Bool(true),Output::Num(87),Output::Num(74),Output::Num(87),Output::Bool(true),Output::Num(85),Output::Bool(true),Output::Bool(true),Output::Bool(false),
        Output::Num(74),Output::Num(74),Output::Num(74),Output::Num(74),Output::Bool(true),Output::Num(74),Output::Bool(false),Output::Bool(true),Output::Bool(true),Output::Bool(true),
        Output::Bool(true)];

    let errors = leetcode_test(k, funcs, inputs, outputs);
    if errors > 0 {
        println!("Failed test 3 with {} errors!", errors);
    } else {
        println!("Passed test 3!");
    }
}

fn main() {
    test1();
    test2();
    test3();
    println!("All tests passed!");
}
