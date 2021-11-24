
// https://leetcode.com/problems/palindrome-number/
// Given an integer x, return true if x is palindrome integer.

// An integer is a palindrome when it reads the same backward as forward.
// For example, 121 is palindrome while 123 is not.


impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || x % 10 == 0 && x != 0{
            return false;
        }
        
        let mut n = 0;
        let mut y = x;
        
        while n < y {
            n = n * 10 + y % 10;
            y = y / 10;
            
        }
        return y == n || y == n / 10;
        
    }
}
