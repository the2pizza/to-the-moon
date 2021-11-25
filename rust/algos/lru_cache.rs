
// https://leetcode.com/problems/lru-cache/

// the code is not working for LeetCode runtime because the compiler is old. 

use std::collections::{VecDeque, HashMap};

#[derive(Debug)]
struct LRUCache {
    store: Option<Box<HashMap<i32, i32>>>,
    queue: Option<VecDeque<i32>>,
    capacity: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    
    fn new(capacity: i32) -> Self {    
        LRUCache {
            store: Some(Box::new(HashMap::new())),
            queue: Some(VecDeque::new()),
            capacity: capacity as usize
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        match (self.store.clone(), self.queue.clone()) {
            (Some(s), Some(mut q)) => {
                return match s.get(&key) {
                    Some(v) => {
                        
                        if q.contains(&key) {
                            let i = q.binary_search(&key);
                            q.remove(i.unwrap());
                        }
                        
                        q.push_front(key);
                        self.queue = Some(q);  
                        *v
                    } 
                    None => {
                        -1
                    } 
                }
            }
            _ => {
                -1
            }
        }       
    }
    
    fn put(&mut self, key: i32, value: i32) {
       
        match (self.store.clone(), self.queue.clone()) {
            (Some(mut s), Some(mut q)) => {
               if s.len() >= self.capacity && ! s.contains_key(&key) {
                   
                   let last = q.pop_back().unwrap();
                   s.remove(&last);  
                }
                
                if q.contains(&key) {
                    let i = q.binary_search(&key);
                    q.remove(i.unwrap());
                }
                
                s.insert(key, value);
                q.push_front(key);
                
                self.store = Some(Box::new(*s));
                self.queue = Some(q);
                
                return ();    
            }
            _ => {
                ()
            }
        }       
    }        
}


/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */


fn main() {

    let mut lru = LRUCache::new(2);
    
    lru.put(1,1);
    println!("{:?}", lru);
    lru.put(2,2);
    println!("{:?}", lru);
    lru.get(1);
    println!("{:?}", lru);
    lru.put(3,3);
    println!("{:?}", lru);
    lru.get(2);
    println!("{:?}", lru);
    lru.put(4,4);
    println!("{:?}", lru);
    lru.get(1);
    println!("{:?}", lru);
    lru.get(3);
    println!("{:?}", lru);
    lru.get(4);
    
    println!("{:?}", lru);
    
    

}
