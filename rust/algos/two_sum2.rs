/* The task https://leetcode.com/problems/two-sum/

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
*/

use std::collections::HashMap; 

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut index:HashMap<i32, i32> = HashMap::new();
        let mut i = 0;
        
        return loop {
            let c = target - nums[i];
            
            match index.get(&c) {
                Some(v) => {
                    break vec![*v, i as i32];
                }
                None => {
                    index.insert(nums[i], i as i32);
                }
                
            };
            i += 1;   
        };        
    }
}
