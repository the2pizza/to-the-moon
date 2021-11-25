// https://leetcode.com/problems/lru-cache/

use std::collections::{VecDeque, HashMap};

struct LRUCache {
    store: HashMap<i32, i32>,
    queue: VecDeque<i32>,
    capacity: usize
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    
    fn new(capacity: i32) -> Self {    
        LRUCache {
            store: HashMap::new(),
            queue: VecDeque::new(),
            capacity: capacity as usize
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        return match self.store.get(&key) {
            Some(v) => {
                if self.queue.contains(&key) {
                    let i = self.queue.iter().position(|n| n == &key);
                    self.queue.remove(i.unwrap());
                }
                
                self.queue.push_front(key); 
                *v
            } 
            None => {
                -1
            } 
        }
    }       
    
    
    fn put(&mut self, key: i32, value: i32) {
       
      
                if self.store.len() >= self.capacity && ! self.store.contains_key(&key) {
                   let last = self.queue.pop_back();
                   self.store.remove(&last.unwrap());  
                }
                
                if self.queue.contains(&key) {
                    let n = self.queue.iter().position(|i| i == &key);
                    self.queue.remove(n.unwrap());
                }
                
                self.store.insert(key, value);
                self.queue.push_front(key);
       
                return ();    
          
        }       
           
}


/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */