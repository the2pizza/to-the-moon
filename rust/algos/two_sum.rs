use std::collections::HashMap; 

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut index:HashMap<i32, i32> = HashMap::new();
        let mut i = 0;
        
        return loop {
            let mut c = target - nums[i];
            if index.contains_key(&c) {
                let v = index.get(&c);
                
                break vec![*v.unwrap(), i as i32];
            }
            
            index.insert(nums[i], i as i32);
            i += 1;   
        };        
    }
}
