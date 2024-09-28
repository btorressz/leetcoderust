struct MyCircularDeque {
   data: Vec<i32>,
    front: usize,
    rear: usize,
    size: usize,
    capacity: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        MyCircularDeque {
            data: vec![0; k as usize],
            front: 0,
            rear: 0,
            size: 0,
            capacity: k as usize,
        }
    }
    
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.front = (self.front + self.capacity - 1) % self.capacity;
        self.data[self.front] = value;
        self.size += 1;
        true
    }
    
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
        self.size += 1;
        true
    }
    
    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        true
    }
    
    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.rear = (self.rear + self.capacity - 1) % self.capacity;
        self.size -= 1;
        true
    }
    
    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.front]
        }
    }
    
    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let rear_index = (self.rear + self.capacity - 1) % self.capacity;
            self.data[rear_index]
        }
    }
    
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
