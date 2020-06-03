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
        
        for i in (self.rhead..self.size).rev() {
            let prev = (i - 1) as usize;
            self.vals[prev] = self.vals[i as usize];
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
            // No need to shift, just forget rear "head" value
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
            // No need to shift, just forget front "tail" value
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

fn test1() {
    let mut cdq = MyCircularDeque::new(3);      // ^[_,_,_]$ (vals with ftail ^ and rhead $)
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
    println!("{:?}", cdq.vals);
    assert!(cdq.get_rear() == 9);               // [9,9^,_]$
    println!("Test 2 passed");
}

fn main() {
    test1();
    test2();
    println!("All tests passed!");
}
